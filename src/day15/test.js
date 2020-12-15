import { part1, part2 } from './index.js';

test('part1', () => {
  expect(part1('0,3,6')).toBe(436);
  expect(part1('5,1,9,18,13,8,0')).toBe(376);
});

test('part2', () => {
  expect(part2('5,1,9,18,13,8,0')).toBe(323_780);
});
