const rulesRegex = /(.+): (\d+)-(\d+) or (\d+)-(\d+)/g;
const ticketRegex = /^(?:\d+,?)+$/g;

/**
 * @param {string} input
 */
function* parseRules(input) {
  for (const [_, name, aRaw, bRaw, cRaw, dRaw] of input.matchAll(rulesRegex)) {
    const a = Number(aRaw);
    const b = Number(bRaw);
    const c = Number(cRaw);
    const d = Number(dRaw);

    /**
     * @param {number} value
     */
    function matches(value) {
      return (a <= value && value <= b) || (c <= value && value <= d);
    }

    yield { name, matches };
  }
}

/**
 * @param {string} input
 */
function parseInput(input) {
  const rules = Array.from(parseRules(input));
  const [myTicket, ...nearbyTickets] = input
    .trim()
    .split('\n')
    .filter(line => line.match(ticketRegex))
    .map(line => line.split(',').map(Number));
  return { rules, myTicket, nearbyTickets };
}

/**
 * @param {string} input
 */
export function part1(input) {
  const { rules, nearbyTickets } = parseInput(input);

  return nearbyTickets
    .flat(1)
    .filter(value => !rules.some(rule => rule.matches(value)))
    .reduce((a, b) => a + b, 0);
}

/**
 * @param {number} n
 */
function isPowerOfTwo(n) {
  return (n & (n - 1)) === 0;
}

/**
 * @param {string} input
 */
export function part2(input) {
  const { rules, myTicket, nearbyTickets } = parseInput(input);

  const problemSize = rules.length;

  const validNearbyTickets = nearbyTickets.filter(ticket =>
    ticket.every(value => rules.some(rule => rule.matches(value))),
  );

  const allowedRulesPerPosition = Array.from({ length: problemSize }, (_, position) =>
    rules.reduce(
      (allowedRules, rule, ruleIndex) =>
        validNearbyTickets.every(ticket => rule.matches(ticket[position]))
          ? allowedRules | (1 << ruleIndex)
          : allowedRules,
      0,
    ),
  );

  let visitedPositions = 0;
  let currentPosition = -1;

  while (
    (currentPosition = allowedRulesPerPosition.findIndex(
      (rules, position) => isPowerOfTwo(rules) && (visitedPositions & (1 << position)) === 0,
    )) >= 0
  ) {
    visitedPositions |= 1 << currentPosition;

    for (let otherPosition = 0; otherPosition < problemSize; otherPosition += 1) {
      if (otherPosition !== currentPosition) {
        allowedRulesPerPosition[otherPosition] &= ~allowedRulesPerPosition[currentPosition];
      }
    }
  }

  /**
   * @param {number[] | undefined} visitedRulePositions
   * @returns {number[] | undefined}
   */
  function findRulePositions(visitedRulePositions = []) {
    let currentPosition = visitedRulePositions.length;

    if (currentPosition === problemSize) {
      return visitedRulePositions;
    } else {
      const allowedRules = allowedRulesPerPosition[currentPosition];

      for (let rulePosition = 0; rulePosition < problemSize; rulePosition += 1) {
        if (
          (allowedRules & (1 << rulePosition)) !== 0 &&
          !visitedRulePositions.includes(rulePosition)
        ) {
          const rulePositions = findRulePositions([...visitedRulePositions, rulePosition]);
          if (rulePositions !== undefined) {
            return rulePositions;
          }
        }
      }

      return undefined;
    }
  }

  const rulePositions = findRulePositions();

  if (rulePositions === undefined) {
    throw new Error('No rule positions found!');
  }

  return myTicket
    .filter((_, ruleIndex) => rules[rulePositions[ruleIndex]].name.startsWith('departure'))
    .reduce((a, b) => a * b, 1);
}
