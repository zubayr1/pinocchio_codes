/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import { State, stateBeet } from '../types/State'

/**
 * Arguments used to create {@link MyState}
 * @category Accounts
 * @category generated
 */
export type MyStateArgs = {
  isInitialized: number
  owner: web3.PublicKey
  state: State
  data: number[] /* size: 32 */
  updateCount: number
  bump: number
}
/**
 * Holds the data for the {@link MyState} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class MyState implements MyStateArgs {
  private constructor(
    readonly isInitialized: number,
    readonly owner: web3.PublicKey,
    readonly state: State,
    readonly data: number[] /* size: 32 */,
    readonly updateCount: number,
    readonly bump: number
  ) {}

  /**
   * Creates a {@link MyState} instance from the provided args.
   */
  static fromArgs(args: MyStateArgs) {
    return new MyState(
      args.isInitialized,
      args.owner,
      args.state,
      args.data,
      args.updateCount,
      args.bump
    )
  }

  /**
   * Deserializes the {@link MyState} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [MyState, number] {
    return MyState.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link MyState} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<MyState> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find MyState account at ${address}`)
    }
    return MyState.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'ENrRns55VechXJiq4bMbdx7idzQh7tvaEJoYeWxRNe7Y'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, myStateBeet)
  }

  /**
   * Deserializes the {@link MyState} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [MyState, number] {
    return myStateBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link MyState} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return myStateBeet.serialize(this)
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link MyState}
   */
  static get byteSize() {
    return myStateBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link MyState} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      MyState.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link MyState} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === MyState.byteSize
  }

  /**
   * Returns a readable version of {@link MyState} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      isInitialized: this.isInitialized,
      owner: this.owner.toBase58(),
      state: 'State.' + State[this.state],
      data: this.data,
      updateCount: this.updateCount,
      bump: this.bump,
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const myStateBeet = new beet.BeetStruct<MyState, MyStateArgs>(
  [
    ['isInitialized', beet.u8],
    ['owner', beetSolana.publicKey],
    ['state', stateBeet],
    ['data', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['updateCount', beet.u32],
    ['bump', beet.u8],
  ],
  MyState.fromArgs,
  'MyState'
)
