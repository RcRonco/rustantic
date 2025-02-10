# Missing to_rs function
import enum
from typing import Annotated, Literal, Union
from pydantic import BaseModel, Discriminator, Field, RootModel
from rustantic_test.generated.nested import Nested


class MyEnumDiscriminator(enum.Enum):
    A = enum.auto()
    B = enum.auto()
    C = enum.auto()


class MyEnumA(BaseModel):
    kind: Literal[MyEnumDiscriminator.A] = Field(default=MyEnumDiscriminator.A, init=False, frozen=True)
    value: Nested


class MyEnumB(BaseModel):
    kind: Literal[MyEnumDiscriminator.B] = Field(default=MyEnumDiscriminator.B, init=False, frozen=True)
    value: Nested


class MyEnumC(BaseModel):
    kind: Literal[MyEnumDiscriminator.C] = Field(default=MyEnumDiscriminator.C, init=False, frozen=True)
    value: int


MyEnumType = Union[MyEnumA,MyEnumB,MyEnumC]

class MyEnum(RootModel[MyEnumType]):
    root: MyEnumType = Field(..., discriminator="kind")