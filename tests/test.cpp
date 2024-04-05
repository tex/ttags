class Class_1
{
  public:
    Class_1() : m_Variable_1(0), m_Struct_1() { };
    Setup() { m_Variable_1 = 10; };
    Work();
  private:
    int m_Variable_1;
    MyStruct m_Struct_1;
};

Class_1::Work()
{
  int local_1 = 0;
  while (m_Variable_1) {
    m_Struct_1.Test();
  }

  MyStruct myStruct;
  myStruct.Todo();
}

char GLOBAL_ARRAY[] = "Hello";

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
}


