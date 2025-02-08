# Missing to_rs function
import enum
from typing import Annotated, Literal, Union
from pydantic import BaseModel, Discriminator, Field, RootModel

class MyEnumDiscriminator(enum.Enum):
    A = enum.auto()
    B = enum.auto()
    C = enum.auto()


class MyEnumA(BaseModel):
    kind: Literal[MyEnumDiscriminator.A] = Field(default=MyEnumDiscriminator.A, init=False, frozen=True)
    value: Nested # Type unresolved!


class MyEnumB(BaseModel):
    kind: Literal[MyEnumDiscriminator.B] = Field(default=MyEnumDiscriminator.B, init=False, frozen=True)
    value: Nested # Type unresolved!


class MyEnumC(BaseModel):
    kind: Literal[MyEnumDiscriminator.C] = Field(default=MyEnumDiscriminator.C, init=False, frozen=True)
    value: i16 # Type unresolved!


MyEnumType = Union[MyEnumA,MyEnumB,MyEnumC]
MyEnum = Annotated[MyEnumType, Discriminator("kind")]

class MyEnumRootModel(RootModel[MyEnumType]):
    root: MyEnumType = Field(..., discriminator="kind")