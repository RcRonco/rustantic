# Generated by rustantic version: 0.2.0
from pydantic import BaseModel, Field
from pydantic import Field
import rustantic_test

class Nested2(BaseModel):
    name: str
    num: int = Field(..., ge=0)
