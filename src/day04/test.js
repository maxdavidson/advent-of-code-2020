import fs from 'fs/promises';

import { part1, part2 } from './index.js';

const [INPUT, TEST_INPUT_0, TEST_INPUT_1, TEST_INPUT_2] = await Promise.all([
  fs.readFile(new URL('input.txt', import.meta.url), 'utf-8'),
  fs.readFile(new URL('test-input-0.txt', import.meta.url), 'utf-8'),
  fs.readFile(new URL('test-input-1.txt', import.meta.url), 'utf-8'),
  fs.readFile(new URL('test-input-2.txt', import.meta.url), 'utf-8'),
]);

test('part1', () => {
  expect(part1(TEST_INPUT_0)).toBe(2);
  expect(part1(INPUT)).toBe(192);
});

test('part2', () => {
  expect(part2(TEST_INPUT_1)).toBe(0);
  expect(part2(TEST_INPUT_2)).toBe(4);
  expect(part2(INPUT)).toBe(101);
});
