#include <stdint.h>
#include <stdio.h>

int8_t quantize_float32_to_int8(float input) {
  if (input >= 1.0)
    return 127;
  else if (input <= 0.0)
    return -128;
  return (int8_t)(input * 255.0f) - 128;
}

void quantize_float32_to_int8_array(const float *restrict input,
                                    int8_t *restrict out, int len) {
  for (int i = 0; i < len; i++)
    out[i] = quantize_float32_to_int8(input[i]);
}

#define N 10

int main() {
  float input[N];
  int8_t out[N];

  for (int i = 0; i < N; i++)
    input[i] = 0.5;

  quantize_float32_to_int8_array(input, out, N);

  for (int i = 0; i < N; i++)
    printf("%d,", out[i]);
  printf("\n");
}
