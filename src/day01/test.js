import fs from 'fs/promises';

import { part1, part2 } from './index.js';

const [INPUT, TEST_INPUT] = await Promise.all([
  fs.readFile(new URL('input.txt', import.meta.url), 'utf-8'),
  fs.readFile(new URL('test-input.txt', import.meta.url), 'utf-8'),
]);

test('part1', () => {
  expect(part1(TEST_INPUT)).toBe(514_579);
  expect(part1(INPUT)).toBe(1_019_371);
});

test('part2', () => {
  expect(part2(TEST_INPUT)).toBe(241_861_950);
  expect(part2(INPUT)).toBe(278_064_990);
});
