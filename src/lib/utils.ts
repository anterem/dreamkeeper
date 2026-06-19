import type { Schedule } from './bindings';

function to12HourTime(h: number): { hour: number; meridiem: 'AM' | 'PM' } {
  return {
    hour: h % 12 === 0 ? 12 : h % 12,
    meridiem: h % 24 < 12 ? 'AM' : 'PM'
  };
}

function formatHourRange({ start, end }: Schedule): string {
  const s = to12HourTime(start);
  const e = to12HourTime(end);
  return (
    `${s.hour}` + (s.meridiem === e.meridiem ? '' : ` ${s.meridiem}`) + ` – ${e.hour} ${e.meridiem}`
  );
}

export function formatSchedule(daySchedule: Schedule[]): string {
  if (daySchedule.length === 0) return '—';
  if (daySchedule.length === 1) {
    const { start, end } = daySchedule[0];
    if (start === 0 && end === 24) return 'All day';
    if (start === 0 && end === 12) return 'Morning';
    if (start === 12 && end === 24) return 'Afternoon';
  }
  return daySchedule.map(formatHourRange).join(', ');
}

export const WEEKDAY_NAMES = [
  'Sunday',
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday'
];

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
