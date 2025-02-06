# Generated by rustantic version: 0.2.0
from pydantic import BaseModel
import rustantic_test
from pydantic import Field
from rustantic_test.generated.nested import Nested as PydanticNested

class MyClass(BaseModel):
    name: str
    num2: int = Field(..., ge=0)
    nested: PydanticNested

    def to_rs(self):
        return rustantic_test.MyClass(
            name=self.name,
            num2=self.num2,
            nested=self.nested.to_rs(),
        )