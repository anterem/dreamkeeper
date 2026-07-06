import type { Critter, PreferredGift, Schedule, Villager } from './bindings';

const DAY = 86400;
const HOUR = 3600;

function localSecs(nowSecs: number, tzOffset: number): number {
  return nowSecs + tzOffset;
}

// 0 = Sunday, matches critter schedule array
export function localWeekday(nowSecs: number, tzOffset: number): number {
  const days = Math.floor(localSecs(nowSecs, tzOffset) / DAY);
  return (((days + 4) % 7) + 7) % 7;
}

function localHour(nowSecs: number, tzOffset: number): number {
  const secs = ((localSecs(nowSecs, tzOffset) % DAY) + DAY) % DAY;
  return Math.floor(secs / HOUR);
}

function localMidnight(nowSecs: number, tzOffset: number): number {
  return Math.floor(localSecs(nowSecs, tzOffset) / DAY) * DAY;
}

// gifts reset at 5am local
function last5amUtc(nowSecs: number, tzOffset: number): number {
  const local = localSecs(nowSecs, tzOffset);
  const local5am = Math.floor((local - 5 * HOUR) / DAY) * DAY + 5 * HOUR;
  return local5am - tzOffset;
}

function isAvailable(schedule: Schedule[], hour: number): boolean {
  return schedule.some((s) => hour >= s.start && hour < s.end);
}

export type LiveCritter = Critter & {
  availableNow: boolean;
  fedToday: boolean;
  needsFeeding: boolean;
};

export function liveCritter(critter: Critter, nowSecs: number, tzOffset: number): LiveCritter {
  const availableNow = isAvailable(
    critter.schedule[localWeekday(nowSecs, tzOffset)],
    localHour(nowSecs, tzOffset)
  );
  // lastFeedingSecs is in the same local-as-UTC space as localMidnight
  const fedToday =
    critter.lastFeedingSecs !== null && critter.lastFeedingSecs >= localMidnight(nowSecs, tzOffset);
  return {
    ...critter,
    availableNow,
    fedToday,
    needsFeeding: availableNow && !critter.tamed && !fedToday
  };
}

export type LiveGift = PreferredGift & { giftedToday: boolean };
export type LiveVillager = Omit<Villager, 'gifts'> & {
  giftableToday: boolean;
  needsGifting: boolean;
  gifts: LiveGift[];
};

export function liveVillager(villager: Villager, nowSecs: number, tzOffset: number): LiveVillager {
  const giftableToday =
    villager.lastGiftSecs === null || villager.lastGiftSecs < last5amUtc(nowSecs, tzOffset);
  const gifts = villager.gifts.map((g) => ({ ...g, giftedToday: !giftableToday && g.gifted }));
  return {
    ...villager,
    giftableToday,
    gifts,
    needsGifting: villager.status === 'inVillage' && gifts.some((g) => !g.giftedToday)
  };
}
