import { appendTransactionMessageInstruction, pipe } from '@solana/web3.js';
import test from 'ava';
import { getPingInstruction } from '../src';
import {
  createDefaultSolanaClient,
  createDefaultTransaction,
  generateKeyPairSignerWithSol,
  signAndSendTransaction,
} from './_setup';

test('it can ping the program', async (t) => {
  const client = createDefaultSolanaClient();
  const payer = await generateKeyPairSignerWithSol(client);

  const pingIx = getPingInstruction({});
  await pipe(
    await createDefaultTransaction(client, payer),
    (tx) => appendTransactionMessageInstruction(pingIx, tx),
    (tx) => signAndSendTransaction(client, tx)
  );

  t.assert(true);
});
