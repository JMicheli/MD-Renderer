#include <consts.glsl>

// Normal Distribution Function - GGX/Trowbridge-Reitz Distribution
// Describes the statistical distribution of microfacet orientations
float ggx_distribution(float NdotH, float roughness) {
  float a = roughness * roughness; // Remapping for more perceptual linearity
  float a2 = a * a;
  float numer = a2; // Numerator: concentration factor

  float NdotH2 = NdotH * NdotH;
  float denom = (NdotH2 * (a2 - 1.0) + 1.0);
  denom = PI * denom * denom; // Normalization factor

  return numer / denom; // Normalized distribution
}

// Shadow Masking Function - Smith's method with Schlick-GGX approximation
// Models self-shadowing and masking between microfacets
float smith_shadow_masking(float NdotV, float NdotL, float roughness) {
  float r = roughness + 1.0;
  float k = (r * r) / 8.0; // Direct lighting remapping

  // Geometry obstruction from view direction (masking)
  float ggx1 = NdotV / (NdotV * (1.0 - k) + k);
  // Geometry obstruction from light direction (shadowing)
  float ggx2 = NdotL / (NdotL * (1.0 - k) + k);

  return ggx1 * ggx2; // Combined masking-shadowing
}

// Fresnel Reflectance - Schlick's approximation
// Models how reflectance changes with viewing angle
vec3 schlick_fresnel(float cos_theta, vec3 f0) {
  return f0 + (1.0 - f0) * pow(1.0 - cos_theta, 5.0);
}
