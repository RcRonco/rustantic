import rustantic_test
from rustantic_test.generated.my_enum import MyEnum, MyEnumA, MyEnumB, MyEnumC, MyEnumD
from rustantic_test.generated.nested_2 import Nested2

def test_my_enum():
    a = MyEnum(root=MyEnumA(value=Nested2(name="name", num=1)))
    b = MyEnum(root=MyEnumB(value=Nested2(name="name", num=1)))
    c = MyEnum(root=MyEnumC(value=1))
    d = MyEnum(root=MyEnumD())

    assert(rustantic_test.check_my_enum(a.to_rs()) == "A")
    assert(rustantic_test.check_my_enum(b.to_rs()) == "B")
    assert(rustantic_test.check_my_enum(c.to_rs()) == "C")
    assert(rustantic_test.check_my_enum(d.to_rs()) == "D")
