#version 450 core

layout(location = 0) in vec3 vertex_pos;
layout(location = 1) in vec3 vertex_normal;

out vec3 model_pos;
out vec3 frag_pos;
out vec3 frag_normal;

layout(std430, binding = 0) buffer uniforms
{
    mat4 vp;
    vec4 time_instance_count;
};

mat4 translate(vec3 v) {
    return mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        v.x, v.y, v.z, 1.0
    );
}

mat4 scale(vec3 v) {
    return mat4(
        v.x, 0.0, 0.0, 0.0,
        0.0, v.y, 0.0, 0.0,
        0.0, 0.0, v.z, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
}

mat4 rotate(float angle, vec3 axis) {
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;
    
    return mat4(
        oc * axis.x * axis.x + c,          oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
        oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
        oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
        0.0,                               0.0,                                0.0,                                1.0
    );
}

mat4 random_rotation(int instanceID) {
    float angle = 2.0 * 3.14159265 * fract(sin(dot(vec2(instanceID, instanceID), vec2(12.9898, 78.233))) * 43758.5453);
    return rotate(angle, vec3(0.0, 1.0, 0.0));
}

vec3 randomDirection(int instanceID) {
    float angle = 2.0 * 3.14159265 * fract(sin(dot(vec3(instanceID), vec3(12.9898, 78.233, 45.164))) * 43758.5453);
    vec3 dir = normalize(vec3(cos(angle), fract(sin(float(instanceID) * 93.9898) * 43758.5453) * 2.0 - 1.0, sin(angle)));
    return dir;
}

mat4 calculateInstanceMatrix(int instanceID) {
    float b = 10.0;
    float a = 0.3;
    float f = 5.0 * b;
    float s = 0.02;
    float rs = 0.1;

    int len = int(time_instance_count.y) / 2;
    int halfInstanceID = instanceID % len;
    float midY = b * 0.5 - b / 2.0;
    float offset = step(len, instanceID) * 180.0;
    
    float t = float(halfInstanceID) / float(len);

    float sin_ft = sin(f * t); 
    float cos_ft = cos(f * t);

    float x = a * cos_ft;
    float z = a * sin_ft;
    float y = b * t - b / 2.0;

    // Generate pseudo-random offsets based on instanceID
    float offset_x = fract(sin(float(instanceID) * 78.233) * 43758.5453) * rs;
    float offset_y = fract(sin(float(instanceID) * 92.993) * 43758.5453) * rs;
    float offset_z = fract(sin(float(instanceID) * 54.734) * 43758.5453) * rs;

    // Compression effect over time
    float timeEffect = (sin(time_instance_count.x) + 1.0) * 0.5; // Oscillate [0,1]
    
    mat4 trans;
    if (time_instance_count.z < 1.0f)
    {
        float yPosAdjustment;
        if (timeEffect <= 0.5) {
            // Before and at the peak of contraction
            yPosAdjustment = (y - midY) * (2.0 * timeEffect); // Amplify effect towards 0.5
        } else {
            // After the peak, reduce the effect back to normal
            yPosAdjustment = (y - midY) * (2.0 * (1.0 - timeEffect)); // Diminish effect after 0.5 
        }

        trans = translate(vec3(x + offset_x, y + offset_y - yPosAdjustment, z + offset_z));
    }
    
    // Apply a random direction based on timeEffect for explosion lasting until 1.0
    else if (time_instance_count.z < 2.0f) {

        float timeEffect = (sin(time_instance_count.w) + 1.0) * 0.5;

        float yPosAdjustment;
        if (timeEffect <= 0.5) {
            // Before and at the peak of contraction
            yPosAdjustment = (y - midY) * (2.0 * timeEffect); // Amplify effect towards 0.5
        } else {
            // After the peak, reduce the effect back to normal
            yPosAdjustment = (y - midY) * (2.0 * (1.0 - timeEffect)); // Diminish effect after 0.5 
        }

        trans = translate(vec3(x + offset_x, y + offset_y - yPosAdjustment, z + offset_z));

        float explosionProgress = (timeEffect - 0.5) * 2.0; // Scale from 0.0 to 1.0 after the midpoint
        vec3 direction = randomDirection(instanceID);
        float forceMagnitude = max(0.0, 1.0 - explosionProgress); // Decreases after explosion
        vec3 explosionOffset = direction * forceMagnitude * 0.65; // Adjust multiplier for effect intensity
        trans *= translate(explosionOffset);
    }

    // Basic transformation matrices
    mat4 rot = rotate(offset + 4.0 * time_instance_count.x, vec3(0.0, 1.0, 0.0));
    mat4 randRot = random_rotation(instanceID);
    mat4 scaleMat = scale(vec3(s, s, s));

    return rot * trans * randRot * scaleMat;
}

void main()
{
    float time = time_instance_count.x;
    mat4 instanceRot = mat4(
        sin(time), cos(time), 0.0, 0.0,
        cos(time), -sin(time), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0);

    mat4 transformDna = calculateInstanceMatrix(gl_InstanceID);
    mat4 transform = transformDna * instanceRot;
    mat3 normal_transform = mat3(transpose(inverse(transform)));
    
    vec4 pos = transform * vec4(vertex_pos, 1.0);
    vec3 normal = normal_transform * vertex_normal;

    model_pos = vertex_pos.xyz;
    frag_pos = pos.xyz;
    frag_normal = normal;
    gl_Position = vp * pos;
}