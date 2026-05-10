const rtf = new Intl.RelativeTimeFormat(
  typeof navigator !== 'undefined' ? navigator.language : 'en',
  { numeric: 'auto' }
);

export function formatRelativeTime(epochSecs: number): string {
  if (!Number.isFinite(epochSecs)) return 'unknown';

  const relMs = epochSecs * 1000 - Date.now();
  const absMs = Math.abs(relMs);

  if (absMs < 60_000) return rtf.format(Math.round(relMs / 1_000), 'second');
  if (absMs < 3_600_000) return rtf.format(Math.round(relMs / 60_000), 'minute');
  if (absMs < 86_400_000) return rtf.format(Math.round(relMs / 3_600_000), 'hour');
  if (absMs < 604_800_000) return rtf.format(Math.round(relMs / 86_400_000), 'day');
  if (absMs < 2_592_000_000) return rtf.format(Math.round(relMs / 604_800_000), 'week');
  if (absMs < 31_536_000_000) return rtf.format(Math.round(relMs / 2_592_000_000), 'month');
  return rtf.format(Math.round(relMs / 31_536_000_000), 'year');
}
