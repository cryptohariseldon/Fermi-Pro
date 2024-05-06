import * as fs from 'fs';
import path from "path"


// CONFIGURATION
const INPUT_FILE_PATH = path.join(__dirname,"target","idl","openbook_v2.json")
const OUTPUT_FILE_PATH = path.join(__dirname,"ts",'client','src','openbook_v2.ts')

const replacements = [
  { from: '{"defined":"usize"}', to: '"u64"' },
  { from: '{"defined":"NodeHandle"}', to: '"u32"' },
];

const eventAuthority = {
  name: 'eventAuthority',
  isMut: false,
  isSigner: false,
};

const program = {
  name: 'program',
  isMut: false,
  isSigner: false,
};

const consumeEvents = {
  name: 'consumeEvents',
  docs: [
    'Process up to `limit` [events](crate::state::AnyEvent).',
    '',
    "When a user places a 'take' order, they do not know beforehand which",
    "market maker will have placed the 'make' order that they get executed",
    "against. This prevents them from passing in a market maker's",
    '[`OpenOrdersAccount`](crate::state::OpenOrdersAccount), which is needed',
    'to credit/debit the relevant tokens to/from the maker. As such, Openbook',
    "uses a 'crank' system, where `place_order` only emits events, and",
    '`consume_events` handles token settlement.',
    '',
    'Currently, there are two types of events: [`FillEvent`](crate::state::FillEvent)s',
    'and [`OutEvent`](crate::state::OutEvent)s.',
    '',
    'A `FillEvent` is emitted when an order is filled, and it is handled by',
    'debiting whatever the taker is selling from the taker and crediting',
    'it to the maker, and debiting whatever the taker is buying from the',
    'maker and crediting it to the taker. Note that *no tokens are moved*,',
    "these are just debits and credits to each party's [`Position`](crate::state::Position).",
    '',
    'An `OutEvent` is emitted when a limit order needs to be removed from',
    'the book during a `place_order` invocation, and it is handled by',
    'crediting whatever the maker would have sold (quote token in a bid,',
    'base token in an ask) back to the maker.',
  ],
  accounts: [
    {
      name: 'consumeEventsAdmin',
      isMut: false,
      isSigner: true,
      isOptional: true,
    },
    {
      name: 'market',
      isMut: true,
      isSigner: false,
    },
    {
      name: 'eventHeap',
      isMut: true,
      isSigner: false,
    },
  ],
  args: [
    {
      name: 'limit',
      type: 'u64',
    },
  ],
};

const consumeGivenEvents = {
  name: 'consumeGivenEvents',
  docs: [
    'Process the [events](crate::state::AnyEvent) at the given positions.',
  ],
  accounts: [
    {
      name: 'consumeEventsAdmin',
      isMut: false,
      isSigner: true,
      isOptional: true,
    },
    {
      name: 'market',
      isMut: true,
      isSigner: false,
    },
    {
      name: 'eventHeap',
      isMut: true,
      isSigner: false,
    },
  ],
  args: [
    {
      name: 'slots',
      type: {
        vec: 'u64',
      },
    },
  ],
};

import(INPUT_FILE_PATH)
  .then((jsonData) => {
    const IDL = jsonData;
    const createMarket = IDL.instructions.find(
      (i) => i.name === 'createMarket',
    );

    if (createMarket != null) {
      // remove old create Market
      const withoutOldCreateMarketIxs = IDL.instructions.filter(
        (i) => i.name !== 'createMarket',
      );
      // add new create market
      const modifiedCreateMarket = {
        ...createMarket,
        accounts: [...createMarket.accounts, eventAuthority, program],
      };

      let modifiedIDL = {
        ...IDL,
        instructions: [
          modifiedCreateMarket,
          ...withoutOldCreateMarketIxs,
          consumeEvents,
          consumeGivenEvents,
        ],
      };

      let idlString = JSON.stringify(modifiedIDL);
      replacements.forEach((r) => {
        idlString = idlString.replaceAll(r.from, r.to);
      });

      fs.writeFileSync('./hi.json', idlString);

      modifiedIDL = JSON.parse(idlString);

      const idlTypeContent = `
  export interface OpenbookV2 ${JSON.stringify(modifiedIDL, null, 2)};
  export const Idl: OpenbookV2 = ${JSON.stringify(modifiedIDL, null, 2)};
  `;
      fs.writeFileSync(OUTPUT_FILE_PATH, idlTypeContent);
      console.log('IDL file generated:', OUTPUT_FILE_PATH);
    } else {
      console.error('createMarket not found');
    }
  })
  .catch((error: Error) => {
    console.error(`Error importing JSON file: ${error.message}`);
  });
