/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  containsBytes,
  getU8Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/web3.js';
import { type ParsedPingInstruction } from '../instructions';

export const PING_PROGRAM_PROGRAM_ADDRESS =
  'Ping111111111111111111111111111111111111111' as Address<'Ping111111111111111111111111111111111111111'>;

export enum PingProgramInstruction {
  Ping,
}

export function identifyPingProgramInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): PingProgramInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return PingProgramInstruction.Ping;
  }
  throw new Error(
    'The provided instruction could not be identified as a pingProgram instruction.'
  );
}

export type ParsedPingProgramInstruction<
  TProgram extends string = 'Ping111111111111111111111111111111111111111',
> = {
  instructionType: PingProgramInstruction.Ping;
} & ParsedPingInstruction<TProgram>;
