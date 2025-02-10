# Missing to_rs function
from pydantic import BaseModel, Field, RootModel
from rustantic_test.generated.nested import Nested
from typing import Literal, Union, Any
import enum
import rustantic_test

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
    def to_rs(self):
        inner_to_rs = getattr(self.root.value, "to_rs", lambda v: v)
        val: Any = inner_to_rs(self.root.value)
        match self.root.kind:
            case MyEnumDiscriminator.A:
                return rustantic_test.MyEnum.A(val)

            case MyEnumDiscriminator.B:
                return rustantic_test.MyEnum.B(val)

            case MyEnumDiscriminator.C:
                return rustantic_test.MyEnum.C(val)
