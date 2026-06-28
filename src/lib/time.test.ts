import { describe, it, expect } from 'vitest';
import { liveCritter, liveVillager, localWeekday } from './time';
import type { Critter, PreferredGift, Schedule, Villager } from './bindings';

const H = 3600;
const DAY = 86400;

const SUNDAY = 3 * DAY;
const MONDAY = 4 * DAY;
const TUESDAY = 5 * DAY;

const at = (base: number, hour: number) => base + hour * H;

function schedule(perDay: Record<number, Schedule[]>): Schedule[][] {
  return Array.from({ length: 7 }, (_, day) => perDay[day] ?? []);
}

function critter(overrides: Partial<Critter> = {}): Critter {
  return {
    itemId: 1,
    name: 'Test Critter',
    species: 'squirrel',
    speciesRank: 0,
    biome: 'Peaceful Meadow',
    notes: [],
    schedule: schedule({}),
    tamed: false,
    unlocked: true,
    lastFeedingSecs: null,
    ...overrides
  };
}

function gift(overrides: Partial<PreferredGift> = {}): PreferredGift {
  return {
    itemId: 1,
    name: 'Apple',
    category: 'produce',
    discovered: true,
    gifted: false,
    ...overrides
  };
}

function villager(overrides: Partial<Villager> = {}): Villager {
  return {
    id: 1,
    name: 'Goofy',
    status: 'inVillage',
    role: null,
    friendshipLevel: 1,
    friendshipXp: 0,
    isMaxed: false,
    gifts: [],
    lastGiftSecs: null,
    ...overrides
  };
}

const availableAt = (s: Record<number, Schedule[]>, now: number) =>
  liveCritter(critter({ schedule: schedule(s) }), now, 0).availableNow;

describe('localWeekday', () => {
  it('maps the epoch (Thursday) to index 4', () => {
    expect(localWeekday(0, 0)).toBe(4);
  });

  it('numbers days from Sunday', () => {
    expect(localWeekday(SUNDAY, 0)).toBe(0);
    expect(localWeekday(MONDAY, 0)).toBe(1);
    expect(localWeekday(TUESDAY, 0)).toBe(2);
  });

  it('shifts the day across midnight by timezone', () => {
    expect(localWeekday(at(SUNDAY, 23), 2 * H)).toBe(1);
    expect(localWeekday(at(MONDAY, 0), -H)).toBe(0);
  });
});

describe('critter availability', () => {
  it('an empty schedule is never available', () => {
    expect(availableAt({}, at(SUNDAY, 12))).toBe(false);
  });

  it('a full-day window covers every hour', () => {
    const all = { [0]: [{ start: 0, end: 24 }] };
    expect(availableAt(all, at(SUNDAY, 0))).toBe(true);
    expect(availableAt(all, at(SUNDAY, 12))).toBe(true);
    expect(availableAt(all, at(SUNDAY, 23))).toBe(true);
  });

  it('a morning window spans midnight to noon, end-exclusive', () => {
    const am = { [0]: [{ start: 0, end: 12 }] };
    expect(availableAt(am, at(SUNDAY, 0))).toBe(true);
    expect(availableAt(am, at(SUNDAY, 11))).toBe(true);
    expect(availableAt(am, at(SUNDAY, 12))).toBe(false);
    expect(availableAt(am, at(SUNDAY, 23))).toBe(false);
  });

  it('an afternoon window spans noon to midnight', () => {
    const pm = { [0]: [{ start: 12, end: 24 }] };
    expect(availableAt(pm, at(SUNDAY, 11))).toBe(false);
    expect(availableAt(pm, at(SUNDAY, 12))).toBe(true);
    expect(availableAt(pm, at(SUNDAY, 23))).toBe(true);
  });

  it('respects custom window boundaries', () => {
    const window = { [0]: [{ start: 15, end: 20 }] };
    expect(availableAt(window, at(SUNDAY, 14))).toBe(false);
    expect(availableAt(window, at(SUNDAY, 15))).toBe(true);
    expect(availableAt(window, at(SUNDAY, 19))).toBe(true);
    expect(availableAt(window, at(SUNDAY, 20))).toBe(false);
  });

  it('handles two windows in one day', () => {
    const dual = {
      [0]: [
        { start: 7, end: 8 },
        { start: 19, end: 20 }
      ]
    };
    expect(availableAt(dual, at(SUNDAY, 7))).toBe(true);
    expect(availableAt(dual, at(SUNDAY, 8))).toBe(false);
    expect(availableAt(dual, at(SUNDAY, 12))).toBe(false);
    expect(availableAt(dual, at(SUNDAY, 19))).toBe(true);
    expect(availableAt(dual, at(SUNDAY, 20))).toBe(false);
  });

  it('gates by day of week', () => {
    const tuesdayOnly = { [2]: [{ start: 0, end: 24 }] };
    expect(availableAt(tuesdayOnly, at(SUNDAY, 12))).toBe(false);
    expect(availableAt(tuesdayOnly, at(MONDAY, 12))).toBe(false);
    expect(availableAt(tuesdayOnly, at(TUESDAY, 12))).toBe(true);
  });
});

describe('critter feeding', () => {
  const dayLong = { [0]: [{ start: 0, end: 24 }] };
  const live = (lastFeedingSecs: number | null, tamed = false) =>
    liveCritter(
      critter({ schedule: schedule(dayLong), tamed, lastFeedingSecs }),
      at(SUNDAY, 12),
      0
    );

  it('is unfed when never fed', () => {
    expect(live(null).fedToday).toBe(false);
  });

  it('counts a feeding from today', () => {
    expect(live(SUNDAY).fedToday).toBe(true);
    expect(live(SUNDAY - 1).fedToday).toBe(false);
  });

  it('needs feeding only when available, untamed, and unfed', () => {
    expect(live(null).needsFeeding).toBe(true);
    expect(live(SUNDAY).needsFeeding).toBe(false);
    expect(live(null, true).needsFeeding).toBe(false);
  });
});

describe('villager gifting', () => {
  const giftableAt = (now: number, tz: number, lastGiftSecs: number | null) =>
    liveVillager(villager({ lastGiftSecs }), now, tz).giftableToday;

  it('is giftable when never gifted', () => {
    expect(giftableAt(at(SUNDAY, 12), 0, null)).toBe(true);
  });

  it('resets at the prior 5am before 5am local', () => {
    const cutoff = 5 * H - DAY;
    expect(giftableAt(3 * H, 0, cutoff - 1)).toBe(true);
    expect(giftableAt(3 * H, 0, cutoff)).toBe(false);
  });

  it('resets at today 5am once past it', () => {
    expect(giftableAt(6 * H, 0, 5 * H - 1)).toBe(true);
    expect(giftableAt(6 * H, 0, 5 * H)).toBe(false);
  });

  it('treats exactly 5am as today reset', () => {
    expect(giftableAt(5 * H, 0, 5 * H)).toBe(false);
  });

  it('applies the reset in local time', () => {
    const tz = 10 * H;
    const now = -tz + 2 * H;
    const cutoff = 5 * H - DAY - tz;
    expect(giftableAt(now, tz, cutoff - 1)).toBe(true);
    expect(giftableAt(now, tz, cutoff)).toBe(false);
  });

  it('marks a gift given only after the reset has passed', () => {
    const recent = liveVillager(
      villager({ lastGiftSecs: 6 * H, gifts: [gift({ gifted: true }), gift({ gifted: false })] }),
      6 * H,
      0
    );
    expect(recent.gifts[0].giftedToday).toBe(true);
    expect(recent.gifts[1].giftedToday).toBe(false);

    const stale = liveVillager(
      villager({ lastGiftSecs: 5 * H - DAY, gifts: [gift({ gifted: true })] }),
      6 * H,
      0
    );
    expect(stale.gifts[0].giftedToday).toBe(false);
  });
});
