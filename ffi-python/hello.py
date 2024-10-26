from cffi import FFI

def main():
    print("Hello from ffi-python!")
    ffi = FFI()
    ffi.cdef("""
        int double(int);
    """)

    C = ffi.dlopen("../core-rust/target/release/libconfigler.d")
    print(C.double(9))


if __name__ == "__main__":
    main()
