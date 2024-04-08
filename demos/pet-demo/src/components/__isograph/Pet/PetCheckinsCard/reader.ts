import type {ReaderArtifact, ReaderAst} from '@isograph/react';
import { Pet__PetCheckinsCard__param } from './param_type.ts';
import { Pet__PetCheckinsCard__outputType } from './output_type.ts';
import { PetCheckinsCard as resolver } from '../../../PetCheckinsCard.tsx';

const readerAst: ReaderAst<Pet__PetCheckinsCard__param> = [
  {
    kind: "Scalar",
    fieldName: "id",
    alias: null,
    arguments: null,
  },
  {
    kind: "Linked",
    fieldName: "checkins",
    alias: null,
    arguments: null,
    selections: [
      {
        kind: "Scalar",
        fieldName: "id",
        alias: null,
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "location",
        alias: null,
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "time",
        alias: null,
        arguments: null,
      },
    ],
  },
];

const artifact: ReaderArtifact<
  Pet__PetCheckinsCard__param,
  Pet__PetCheckinsCard__outputType
> = {
  kind: "ReaderArtifact",
  resolver: resolver as any,
  readerAst,
  variant: { kind: "Component", componentName: "Pet.PetCheckinsCard" },
};

export default artifact;
