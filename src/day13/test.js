import fs from 'fs/promises';

import { part1, part2 } from './index.js';

const INPUT = await fs.readFile(new URL('input.txt', import.meta.url), 'utf-8');

test('part1', () => {
  expect(part1('939\n7,13,x,x,59,x,31,19')).toBe(295);
  expect(part1(INPUT)).toBe(222);
});

test('part2', () => {
  expect(part2('0\n17,x,13,19')).toBe(3417);
  expect(part2('0\n67,7,59,61')).toBe(754_018);
  expect(part2('0\n67,x,7,59,61')).toBe(779_210);
  expect(part2('0\n67,7,x,59,61')).toBe(1_261_476);
  expect(part2('0\n1789,37,47,1889')).toBe(1_202_161_486);
  expect(part2(INPUT)).toBe(408_270_049_879_073);
});
