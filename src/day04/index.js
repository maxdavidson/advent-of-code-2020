/**
 * @param {string} input
 */
function passports(input) {
  return input
    .trim()
    .split('\n\n')
    .map(chunks => Object.fromEntries(chunks.split(/\s+/).map(chunk => chunk.split(':'))));
}

const hgtRegex = /^(\d+)(cm|in)$/;
const eclRegex = /^amb|blu|brn|gry|grn|hzl|oth$/;
const hclRegex = /^#[a-f0-9]{6}$/;
const pidRegex = /^\d{9}$/;

/** @type {Record<string, (value: string) => boolean} */
const validators = {
  // (Birth Year) - four digits; at least 1920 and at most 2002.
  byr: val => 1920 <= Number.parseInt(val) && Number.parseInt(val) <= 2002,
  // (Issue Year) - four digits; at least 2010 and at most 2020.
  iyr: val => 2010 <= Number.parseInt(val) && Number.parseInt(val) <= 2020,
  // (Expiration Year) - four digits; at least 2020 and at most 2030.
  eyr: val => 2020 <= Number.parseInt(val) && Number.parseInt(val) <= 2030,
  // (Height) - a number followed by either cm or in:
  // If cm, the number must be at least 150 and at most 193.
  // If in, the number must be at least 59 and at most 76.
  hgt: val => {
    const match = val.match(hgtRegex);
    if (match === null) {
      return false;
    }
    const [_, height, variant] = match;
    switch (variant) {
      case 'cm':
        return 150 <= Number.parseInt(height) && Number.parseInt(height) <= 193;
      case 'in':
        return 59 <= Number.parseInt(height) && Number.parseInt(height) <= 76;
      default:
        return false;
    }
  },
  // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  hcl: val => hclRegex.test(val),
  // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  ecl: val => eclRegex.test(val),
  // (Passport ID) - a nine-digit number, including leading zeroes.
  pid: val => pidRegex.test(val),
};

/**
 * @param {string} input
 */
export function part1(input) {
  const requiredFields = Object.keys(validators);

  /**
   * @param {string} passport
   */
  function hasRequiredFields(passport) {
    return requiredFields.every(field => passport.hasOwnProperty(field));
  }

  return passports(input).filter(hasRequiredFields).length;
}

/**
 * @param {string} input
 */
export function part2(input) {
  const validatorEntries = Object.entries(validators);

  /**
   * @param {Record<string, string>} passport
   */
  function isValid(passport) {
    return validatorEntries.every(([field, validator]) => {
      const value = passport[field];
      return typeof value === 'string' && validator(value);
    });
  }

  return passports(input).filter(isValid).length;
}
