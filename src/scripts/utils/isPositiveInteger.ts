export default function isPositiveInteger(n: number) {
  return !Number.isNaN(n) && Number.isInteger(n) && n > 0;
}
