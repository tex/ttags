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

