
import rustantic_test
from rustantic_test.generated.my_unit_enum import MyUnitEnum

def test_my_unit_enum():
    a = MyUnitEnum.A
    b = MyUnitEnum.B
    c = MyUnitEnum.C
    d = MyUnitEnum.D

    assert(rustantic_test.check_my_unit_enum(a.to_rs()) == "A")
    assert(rustantic_test.check_my_unit_enum(b.to_rs()) == "B")
    assert(rustantic_test.check_my_unit_enum(c.to_rs()) == "C")
    assert(rustantic_test.check_my_unit_enum(d.to_rs()) == "D")
    
    assert(b.value == 300)
    assert(c.value == 900)