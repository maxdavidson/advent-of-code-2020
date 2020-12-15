/** @typedef {{ opcode: string, value: number }} Instruction */

const instructionRegex = /^(\w+) ((?:\+|-)\d+)$/gm;

/**
 * @param {string} input
 */
function* instructions(input) {
  for (const [, opcode, value] of input.matchAll(instructionRegex)) {
    yield { opcode, value: Number.parseInt(value) };
  }
}

/**
 * @param {Instruction[]} program
 */
function runProgram(program) {
  const visited = new Set();
  let pc = 0;
  let acc = 0;

  while (true) {
    const instruction = program[pc];

    if (instruction === undefined) {
      return { terminated: true, result: acc };
    }

    if (visited.has(pc)) {
      return { terminated: false, result: acc };
    }

    visited.add(pc);

    const { opcode, value } = instruction;

    switch (opcode) {
      case 'nop':
        pc += 1;
        break;
      case 'acc':
        acc += value;
        pc += 1;
        break;
      case 'jmp':
        pc += value;
        break;
    }
  }
}

/**
 * @param {string} input
 */
export function part1(input) {
  const program = Array.from(instructions(input));
  const { result } = runProgram(program);
  return result;
}

/**
 * @param {string} input
 */
export function part2(input) {
  const program = Array.from(instructions(input));

  for (let i = 0; i < program.length; ++i) {
    const { opcode, value } = program[i];

    /** @type {Instruction} */
    let newInstruction;
    if (opcode === 'nop') {
      newInstruction = { opcode: 'jmp', value };
    } else if (opcode === 'jmp') {
      newInstruction = { opcode: 'nop', value };
    } else {
      continue;
    }

    const newProgram = program.slice();
    newProgram[i] = newInstruction;

    const { terminated, result } = runProgram(newProgram);

    if (terminated) {
      return result;
    }
  }

  throw new Error('No solution found');
}
