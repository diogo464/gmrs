#define GMMODULE
#define GMOD_ALLOW_DEPRECATED
#include <Interface.h>

using namespace GarrysMod::Lua;

static_assert(SPECIAL_GLOB == 0);
static_assert(SPECIAL_ENV == 1);
static_assert(SPECIAL_REG == 2);

void unused()
{
    (void)Type::Name;
}

extern "C"
{
    int gmod_bridge_top(lua_State* state)
    {
        return LUA->Top();
    }
    void gmod_bridge_push(lua_State* state, int stack_pos)
    {
        return LUA->Push(stack_pos);
    }
    void gmod_bridge_pop(lua_State* state, int amount)
    {
        return LUA->Pop(amount);
    }
    void gmod_bridge_get_table(lua_State* state, int stack_pos)
    {
        return LUA->GetTable(stack_pos);
    }
    void gmod_bridge_get_field(lua_State* state, int stack_pos, const char* name)
    {
        return LUA->GetField(stack_pos, name);
    }
    void gmod_bridge_set_field(lua_State* state, int stack_pos, const char* name)
    {
        return LUA->SetField(stack_pos, name);
    }
    void gmod_bridge_create_table(lua_State* state)
    {
        return LUA->CreateTable();
    }
    void gmod_bridge_set_table(lua_State* state, int i)
    {
        return LUA->SetTable(i);
    }
    void gmod_bridge_set_meta_table(lua_State* state, int i)
    {
        return LUA->SetMetaTable(i);
    }
    bool gmod_bridge_get_meta_table(lua_State* state, int i)
    {
        return LUA->GetMetaTable(i);
    }
    void gmod_bridge_call(lua_State* state, int args, int results)
    {
        return LUA->Call(args, results);
    }
    int gmod_bridge_pcall(lua_State* state, int args, int results, int error_func)
    {
        return LUA->PCall(args, results, error_func);
    }
    int gmod_bridge_equal(lua_State* state, int a, int b)
    {
        return LUA->Equal(a, b);
    }
    int gmod_bridge_raw_equal(lua_State* state, int a, int b)
    {
        return LUA->RawEqual(a, b);
    }
    void gmod_bridge_insert(lua_State* state, int stack_pos)
    {
        return LUA->Insert(stack_pos);
    }
    void gmod_bridge_remove(lua_State* state, int stack_pos)
    {
        return LUA->Remove(stack_pos);
    }
    int gmod_bridge_next(lua_State* state, int stack_pos)
    {
        return LUA->Next(stack_pos);
    }
    void gmod_bridge_throw_error(lua_State* state, const char* error)
    {
        return LUA->ThrowError(error);
    }
    void gmod_bridge_check_type(lua_State* state, int stack_pos, int type)
    {
        return LUA->CheckType(stack_pos, type);
    }
    void gmod_bridge_arg_error(lua_State* state, int arg_num, const char* msg)
    {
        return LUA->ArgError(arg_num, msg);
    }
    void gmod_bridge_raw_get(lua_State* state, int stack_pos)
    {
        return LUA->RawSet(stack_pos);
    }
    void gmod_bridge_raw_set(lua_State* state, int stack_pos)
    {
        return LUA->RawSet(stack_pos);
    }

    const char* gmod_bridge_get_string(lua_State* state, int stack_pos, unsigned int* outlen)
    {
        return LUA->GetString(stack_pos, outlen);
    }
    double gmod_bridge_get_number(lua_State* state, int stack_pos)
    {
        return LUA->GetNumber(stack_pos);
    }
    bool gmod_bridge_get_bool(lua_State* state, int stack_pos)
    {
        return LUA->GetBool(stack_pos);
    }
    CFunc gmod_bridge_get_c_function(lua_State* state, int stack_pos)
    {
        return LUA->GetCFunction(stack_pos);
    }
    void gmod_bridge_get_vector(lua_State* state, int stack_pos, float* vector)
    {
        const auto vec = LUA->GetVector(stack_pos);
        vector[0]      = vec.x;
        vector[1]      = vec.y;
        vector[2]      = vec.z;
    }
    void gmod_bridge_get_angle(lua_State* state, int stack_pos, float* angle)
    {
        const auto ang = LUA->GetAngle(stack_pos);
        angle[0]       = ang.x;
        angle[1]       = ang.y;
        angle[2]       = ang.z;
    }

    void gmod_bridge_push_nil(lua_State* state)
    {
        return LUA->PushNil();
    }
    void gmod_bridge_push_string(lua_State* state, const char* val, unsigned int len)
    {
        return LUA->PushString(val, len);
    }
    void gmod_bridge_push_number(lua_State* state, double val)
    {
        return LUA->PushNumber(val);
    }
    void gmod_bridge_push_bool(lua_State* state, bool val)
    {
        return LUA->PushBool(val);
    }
    void gmod_bridge_push_c_function(lua_State* state, CFunc val)
    {
        return LUA->PushCFunction(val);
    }
    void gmod_bridge_push_c_closure(lua_State* state, CFunc val, int vars)
    {
        return LUA->PushCClosure(val, vars);
    }
    void gmod_bridge_push_vector(lua_State* state, float x, float y, float z)
    {
        auto vec = Vector();
        vec.x    = x;
        vec.y    = y;
        vec.z    = z;
        return LUA->PushVector(vec);
    }
    void gmod_bridge_push_angle(lua_State* state, float p, float y, float r)
    {
        auto ang = Vector();
        ang.x    = p;
        ang.y    = y;
        ang.z    = r;
        return LUA->PushAngle(ang);
    }

    int gmod_bridge_reference_create(lua_State* state)
    {
        return LUA->ReferenceCreate();
    }
    void gmod_bridge_reference_free(lua_State* state, int i)
    {
        return LUA->ReferenceFree(i);
    }
    void gmod_bridge_reference_push(lua_State* state, int i)
    {
        return LUA->ReferencePush(i);
    }
    void gmod_bridge_push_special(lua_State* state, int special)
    {
        return LUA->PushSpecial(special);
    }
    bool gmod_bridge_is_type(lua_State* state, int stack_pos, int ty)
    {
        return LUA->IsType(stack_pos, ty);
    }
    int gmod_bridge_get_type(lua_State* state, int stack_pos)
    {
        return LUA->GetType(stack_pos);
    }
    void* gmod_bridge_new_user_data(lua_State* state, unsigned int size)
    {
        return LUA->NewUserdata(size);
    }
    void* gmod_bridge_get_user_data(lua_State* state, int stack_pos)
    {
        return LUA->GetUserdata(stack_pos);
    }
}
