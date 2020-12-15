import fs from 'fs/promises';

import { part1, part2 } from './index.js';

const INPUT = await fs.readFile(new URL('input.txt', import.meta.url), 'utf-8');

test('part1', () => {
  // expect(boarding_pass_id(boarding_pass("FBFBBFFRLR")).toBe(357);
  // expect(boarding_pass_id(boarding_pass("BFFFBBFRRR")).toBe(567);
  // expect(boarding_pass_id(boarding_pass("FFFBBBFRRR")).toBe(119);
  // expect(boarding_pass_id(boarding_pass("BBFFBBFRLL")).toBe(820);

  expect(part1(INPUT)).toBe(935);
});

test('part2', () => {
  expect(part2(INPUT)).toBe(743);
});
