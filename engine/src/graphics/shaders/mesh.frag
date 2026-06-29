#version 450
#include <data_types.glsl>
#include <consts.glsl>
#include <bsdf.glsl>

// Configuration
// /////////////
#extension GL_EXT_nonuniform_qualifier : enable // TODO - Do we want to keep this?

#define GAMMA_FACTOR 2.2
#define AMBIENT_FACTOR 0.05

// Input buffer objects
// ////////////////////

#include <bind_scene_data.glsl>

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in mat3 v_TBN;

layout(location = 0) out vec4 f_color;


// Material data
layout(set = 1, binding = 0) uniform MdrMeshMaterialData {
  // Base material modifiers
  // ///////////////////////

  // Base color tint/multiplier
  vec4 base_color_factor;
  // Surface roughness multiplier
  float roughness_factor;
  // Metallic property multiplier
  float metallic_factor;
  
  // Texture set bindings (-1 = none)
  // ////////////////////////////////

  // Texture binding index for diffuse color
  int diffuse_texture_set;
  // Texture binding index for metallic/roughness map (standard 
  // is to put metallic in the blue channel and roughness in the
  // green channel).
  int metallic_roughness_texture_set;
  // Texture binding index for normal maps
  int normal_texture_set;
  // Texture binding index for ambient occlusion
  int occlusion_texture_set;
  // Texture binding index for emissive maps
  int emissive_texture_set;
} material;

// Texture maps referenced by material
layout(set = 1, binding = 1) uniform sampler2D textures[];

// Shader Entry Point
// //////////////////
void main() {
  // Determine the material's diffuse color by starting from the
  // base color multiplier and then incorporating any diffuse texture.
  vec4 diffuse_color = material.base_color_factor;
  int idx = material.diffuse_texture_set; 
  if (idx >= 0) {
    diffuse_color *= texture(textures[idx], v_uv);
  }

  // Determine metallic and roughness in the same manner
  // The metallic-roughness map stores metallic in the blue
  // channel and roughness in the green channel.
  float roughness = material.roughness_factor;
  float metallic = material.metallic_factor;
  idx = material.metallic_roughness_texture_set; 
  if (idx >= 0) {
    roughness *= texture(textures[idx], v_uv).y;
    metallic *= texture(textures[idx], v_uv).z;
  }

  // And for ambient occlusion
  float ao = 1.0; // TODO - Is this right?
  idx = material.occlusion_texture_set; 
  if (idx >= 0) {
    ao *= texture(textures[idx], v_uv).x;
  }

  // And for emission
  float emissive = 0.0;
  idx = material.emissive_texture_set; 
  if (idx >= 0) {
    emissive += texture(textures[idx], v_uv).x;
  }

  // Similar process for normal map
  vec3 N = normalize(v_TBN[2]);
  // TODO - Apply normal map here

  // Calculate view direction (fragment to camera)
  vec3 V = normalize(scene_data.camera.position - v_position);

  // Calculate reflection vector for environment mapping
  vec3 R = reflect(-V, N);

  // Calculate F0 (reflectance at normal incidence)
  // Non-metals: low reflectance (~0.04), Metals: colored reflectance from base color
  vec3 F0 = vec3(0.04, 0.04, 0.04);  // Dielectric default
  F0 = mix(F0, diffuse_color.xyz, metallic); // Lerp to metallic behavior

  // Initialize outgoing radiance accumulator
  vec3 Lo = vec3(0.0, 0.0, 0.0);
  for (uint i = 0; i < scene_data.point_light_count; i++) {
    vec3 light_pos = scene_data.point_lights[i].position;
    vec3 light_color = scene_data.point_lights[i].color;
    float light_strength = scene_data.point_lights[i].brightness;

    // Calculate light direction and attenuation
    vec3 L = normalize(light_pos - v_position);                 // Light direction
    float dist_to_light = length(light_pos - v_position);       // Distance for falloff
    float attenuation = 1.0 / (dist_to_light * dist_to_light);  // Inverse square falloff
    vec3 radiance = light_strength * light_color * attenuation;                  // Attenuated light color

    // Calculate half vector (between view and light directions)
    vec3 H = normalize(V + L);

    // BRDF Evaluation
    // ///////////////

    // Calculate all necessary dot products for BRDF terms
    float NdotL = max(dot(N, L), 0.0); // Lambertian falloff
    float NdotV = max(dot(N, V), 0.0); // View angle
    float NdotH = max(dot(N, H), 0.0); // Half vector for specular
    float HdotV = max(dot(H, V), 0.0); // For Fresnel calculation

    // Evaluate Cook-Torrance BRDF components
    float D = ggx_distribution(NdotH, roughness);            // Normal distribution
    float G = smith_shadow_masking(NdotV, NdotL, roughness); // Geometry function
    vec3  F = schlick_fresnel(HdotV, F0);                    // Fresnel reflectance

    // Calculate specular BRDF
    vec3 numerator = D * G * F;
    float denominator = 4.0 * NdotV * NdotL + 0.0001; // Prevent division by zero
    vec3 specular = numerator / denominator;

    // Energy Conservation
    // ///////////////////

    // Fresnel term represents specular reflection ratio
    vec3 kS = F;                        // Specular contribution
    vec3 kD = vec3(1.0, 1.0, 1.0) - kS; // Diffuse contribution (energy conservation)
    kD *= 1.0 - metallic;               // Metals have no diffuse reflection

    // Radiance Accumulation
    // /////////////////////

    // Combine diffuse (Lambertian) and specular (Cook-Torrance) terms
    // Multiply by incident radiance and cosine foreshortening
    Lo += (kD * diffuse_color.xyz / PI + specular) * radiance * NdotL;
  }

  // Ambient and Emissive
  // ////////////////////

  // Add simple ambient lighting (should be replaced with IBL in production)
  vec3 ambient = vec3(0.03, 0.03, 0.03) * diffuse_color.xyz * ao;

  // Combine all lighting contributions
  vec3 color = ambient + Lo + emissive;

  // HDR Tone Mapping and Gamma Correction
  // /////////////////////////////////////

  // Apply Reinhard tone mapping to compress HDR values to [0,1] range
  color = color / (color + vec3(1.0, 1.0, 1.0));

  // Apply gamma correction for sRGB display (inverse gamma)
  float inv_gamma = 1.0 / GAMMA_FACTOR;
  color = pow(color, vec3(inv_gamma, inv_gamma, inv_gamma));

  // Output final color with original alpha
  f_color = vec4(color, diffuse_color.w);
}
