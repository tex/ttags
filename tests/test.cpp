namespace A {
namespace B {
static const uint32_t VER_NAM = 2;
}
}

class Class_1
{
  public:
    Class_1() : m_Variable_1(0), m_Struct_1() { };
    Setup() { m_Variable_1 = 10; };
    Work();
    static const uint32_t VAR = 3u;
  private:
    int &m_rVariable_1;
    int *m_pVariable_1;
    int m_Variable_1;
    int m_aVariable_1[10];
    int *&m_rpVariable1;
    int *m_paVariable_1[10];
    int m_raVariable_1[10];
    int *&m_rpaVariable_1[10];
    int &*m_rpaVariable_2[10];
    MyStruct m_Struct_1;
};

Class_1::Work()
{
  int local_1 = 0;
  extern int extern_1;
  while (m_Variable_1) {
    m_Struct_1.Test();
  }

  MyStruct myStruct;
  myStruct.Todo();
}

char GLOBAL_ARRAY[] = "Hello";
extern char GLOBAL_ARRAY_EXT[];

const uint8_t TEST::MY_CONSTANT = 20;
extern uint8_t OTHER_EXTERN;

void Something() {
  MakeUse(&GLOBAL_ARRAY);
  MakeUse(&GLOBAL_ARRAY[0]);
}

Test::Test() : TestBase(TestObject::Int::TEST_OBJECT_ID) {}

void Something() {
  SOMETHING((uint32_t *)&ARRAY[2], Something);
  someAtributes[0].specificItem = someValue;
}

EXTERN uint16_t platformID;
EXTERN CLASS_ATTRIBUTES     ClassAttributes;
EXTERN INSTANCE_ATTRIBUTES  InstanceAttributes[NUM_INSTANCES];

void Something() {
  switch (g_Global_1 || someFunction() || g_Global_2) {
    case CONST_1:
      break;
    case CONST_2:
      break;
    case 54:
      break;
    case MyClass::CONST:
      break;
  }
    auto a = new Something(YU::MAX_NUM, localVar, g_Global_1);
    return &CallPlatformCalculate<uint16_t, ::Crc::CRC16_CCITT_POLYNOMIAL, ::Crc::SOFTWARE>;
}

template<>
uint32_t Deep::Crc::Calculate<uint32_t, Crc::CRC32_POLYNOMIAL, Crc::SOFTWARE>(const void* pData, size_t sizeBytes, uint32_t seed)
{
    q = T::A::E::SOME;
}

