export function formatTimestamp(secs: number): string {
  return new Date(secs * 1000).toLocaleDateString();
}
