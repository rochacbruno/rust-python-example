#include <Python.h>

uint64_t count_byte_doubles(const char * const data, const uint64_t size) {
    uint64_t count = 0;
    for (uint64_t i = size - 2; i > 0; --i) {
        count += data[i] == data[i + 1];
    }
    return count;
}

PyObject * mycmodule_meth_count_doubles(PyObject * self, PyObject * arg) {
    Py_ssize_t size;
    PyUnicode_AsUTF8AndSize(arg, &size);
    return PyLong_FromUnsignedLongLong(count_byte_doubles(PyUnicode_AsUTF8(arg), size));
}

PyMethodDef module_methods[] = {
    {"count_doubles", (PyCFunction)mycmodule_meth_count_doubles, METH_O, NULL},
    {},
};

PyModuleDef module_def = {PyModuleDef_HEAD_INIT, "mycmodule", NULL, -1, module_methods};

extern "C" PyObject * PyInit_mycmodule() {
    PyObject * module = PyModule_Create(&module_def);
    return module;
}
