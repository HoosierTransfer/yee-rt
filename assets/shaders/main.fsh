#version 410 core
out vec4 FragColor;

#define MAX_DIST 10000.0
#define MAX_BOUNCES 10
#define SAMPLES_PER_PIXEL 10

#define M_PI acos(-1.0)

in vec2 TexCoord;

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform vec3 cameraPosition;
uniform float time;

vec3 getRayDirection(vec2 uv) {
    vec4 clipSpace = vec4(uv * 2.0 - 1.0, 1.0, 1.0);
    vec4 eyeSpace = inverse(projectionMatrix) * clipSpace;
    eyeSpace.z = -1.0;
    eyeSpace.w = 0.0;
    vec4 worldSpace = inverse(viewMatrix) * eyeSpace;
    return normalize(worldSpace.xyz);
}

uint seed;

vec3 Tonemap_ACES(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return (x * (a * x + b)) / (x * (c * x + d) + e);
}

float random() {
    seed = seed * 747796405u + 2891336453u;
    uint result = ((seed >> ((seed >> 28) + 4u)) ^ seed) * 277803737u;
    result = (result >> 22) ^ result;
    return float(result) / 4294967295.0;
}

vec3 randomUnitVector() {
    float theta = 2.0 * 3.14159265 * random();
    float phi = acos(2.0 * random() - 1.0);
    return vec3(sin(phi) * cos(theta), sin(phi) * sin(theta), cos(phi));
}

vec3 getHemisphereCosineSample(vec3 n, out float weight) {
    float cosTheta2 = random();
    float cosTheta = sqrt(cosTheta2);
    float sinTheta = sqrt(1. - cosTheta2);
    
    float phi = 2. * M_PI * random();
    
    vec3 t = normalize(cross(n.yzx, n));
    vec3 b = cross(n, t);
    
    vec3 l = (t * cos(phi) + b * sin(phi)) * sinTheta + n * cosTheta;
    
    float pdf = (1. / M_PI) * cosTheta;
    weight = (.5 / M_PI) / (pdf + 1e-6);
    
    return l;
}

vec2 randomInUnitDisk() {
    float r = sqrt(random());
    float theta = 2.0 * 3.14159265 * random();
    return r * vec2(cos(theta), sin(theta));
}

struct Material {
    vec3 albedo;
    float roughness;
    bool isMetal;
    bool isDielectric;
    float indexOfRefraction;
};

struct HitInfo {
    float dist;
    vec3 normal;
    bool frontFace;
    bool didHit;
    Material material;
};

vec3 skyBox(vec3 rd) {
    vec3 unitDirection = normalize(rd);
    float t = 0.5 * (unitDirection.y + 1.0);
    return (1.0-t)*vec3(1) + t*vec3(0.5, 0.7, 1.0);
}

HitInfo hitSphere(vec3 ro, vec3 rd, vec3 center, float radius) {
    HitInfo info;
    vec3 oc = ro - center;
    float a = dot(rd, rd);
    float half_b = dot(oc, rd);
    float c = dot(oc, oc) - radius*radius;
    float discr = half_b*half_b - a*c;
    if (discr < 0.) {
        info.didHit = false;
        return info;
    }
    
    float dist = (-half_b - sqrt(discr)) / a;
    
    if (dist < 0.001) {
        dist = (-half_b + sqrt(discr)) / a;
        if (dist < 0.001) {
            info.didHit = false;
            return info;
        }
    }
    
    info.didHit = true;
    info.dist = dist;
    vec3 outwardNormal = normalize(ro + dist*rd - center);
    info.frontFace = dot(rd, outwardNormal) < 0.;
    info.normal = info.frontFace ? outwardNormal : -outwardNormal;  
    
    return info;
}

HitInfo hitWorld(vec3 ro, vec3 rd) {
    HitInfo info;
    info.didHit = false;
    info.dist = MAX_DIST;
    HitInfo tempInfo;
    tempInfo = hitSphere(ro, rd, vec3(0, -100.5, 0), 100.0);
    if (tempInfo.didHit && tempInfo.dist < info.dist) {
        info = tempInfo;
        info.material = Material(vec3(0.8, 0.8, 0.0), 0.0, false, false, 0.0);
    }
    
    tempInfo = hitSphere(ro, rd, vec3(0, 0, -1), 0.5);
    if (tempInfo.didHit && tempInfo.dist < info.dist) {
        info = tempInfo;
        info.material = Material(vec3(0.1, 0.2, 0.5), 0.0, false, false, 0.0);
    }
    
    tempInfo = hitSphere(ro, rd, vec3(1, 0, -1), 0.5);
    if (tempInfo.didHit && tempInfo.dist < info.dist) {
        info = tempInfo;
        info.material = Material(vec3(0.8, 0.6, 0.2), 1.0, true, false, 0.0);
    }
    
    tempInfo = hitSphere(ro, rd, vec3(-1, 0, -1), 0.5);
    if (tempInfo.didHit && tempInfo.dist < info.dist) {
        info = tempInfo;
        info.material = Material(vec3(1), 0.0, false, true, 1.5);
    }
    
    return info;
}

float schlickFresnel(float cosine, float refractionIndex) {
    float r0 = (1.0 - refractionIndex) / (1.0 + refractionIndex);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow((1.0 - cosine), 5.0);
}

vec3 trace(vec3 rayOrigin, vec3 rayDirection) {
    vec3 ro = rayOrigin;
    vec3 rd = rayDirection;
    
    vec3 col = vec3(0);
    vec3 att = vec3(1);
    for (int i = 0; i < MAX_BOUNCES; i++) {
        HitInfo info = hitWorld(ro, rd);
        if (!info.didHit) {
            col += att * skyBox(rd);
            break;
        }
        
        ro = ro + rd * info.dist;
        
        float cosThetaI = dot(rd, info.normal);
        
        vec3 facingNormal = (cosThetaI < 0.) ? info.normal : -info.normal;
        
        if (info.material.isMetal) {
            vec3 reflected = reflect(rd, info.normal);
            rd = reflected + info.material.roughness * randomUnitVector();
            att *= info.material.albedo;
        } else if (info.material.isDielectric) {
            float refractionRatio = info.frontFace ? (1.0 / info.material.indexOfRefraction) : info.material.indexOfRefraction;
            vec3 unitDirection = normalize(rd); 
            float cosTheta = min(dot(-unitDirection, info.normal), 1.0);
            float sinTheta = sqrt(1.0 - cosTheta*cosTheta);
            
            bool cannotRefract = refractionRatio * sinTheta > 1.0;
            if (cannotRefract || schlickFresnel(cosTheta, refractionRatio) > random()) {
                rd = reflect(unitDirection, info.normal);
            } else {
                rd = refract(unitDirection, info.normal, refractionRatio);
            }
            att *= info.material.albedo;
        } else {
            float weight;
            vec3 reflected = getHemisphereCosineSample(facingNormal, weight);
            
            att *= weight;
            att *= info.material.albedo * dot(facingNormal, reflected);
            
            rd = reflected;
        }
    }
    
    return col;
}

void main() {
    seed = uint(floatBitsToInt(gl_FragCoord.x) + floatBitsToInt(gl_FragCoord.y * 5741.) + floatBitsToInt(time * 26717.));
    
    vec3 color = vec3(0);
    
    vec3 ro = vec3(-2, 2, 1);
    vec3 lookAt = vec3(0, 0, -1);
    
    vec3 rayDirection = getRayDirection(TexCoord);
    for (int i = 0; i < SAMPLES_PER_PIXEL; i++) {
        color += trace(cameraPosition, rayDirection);
    }
    
    color /= float(SAMPLES_PER_PIXEL);
    
    color = Tonemap_ACES(color);
    color = pow(color, vec3(1.0/2.2));
    
    FragColor = vec4(color, 1.0);
}