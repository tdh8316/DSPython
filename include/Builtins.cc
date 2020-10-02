#include "Builtins.hh"

extern "C" int int__f__(float n) {
    return (int)n;
}

extern "C" int int__i__(int n) {
    return (int)n;
}

extern "C" float float__f__(float n) {
    return (float)n;
}

extern "C" float float__i__(int n) {
    return (float)n;
}
