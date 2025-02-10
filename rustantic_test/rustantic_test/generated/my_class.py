# Generated by rustantic version: 0.2.0
from pydantic import BaseModel, Field
from pydantic import Field
from rustantic_test.generated.my_unit_enum import MyUnitEnum
from rustantic_test.generated.nested import Nested
import rustantic_test

class MyClass(BaseModel):
    name: str
    num2: int = Field(..., ge=0)
    nested: Nested
    myenum: MyUnitEnum

    def to_rs(self):
        return rustantic_test.MyClass(
            name=self.name,
            num2=self.num2,
            nested=self.nested.to_rs(),
            myenum=self.myenum.to_rs(),
        )