/*
 * encoding=utf-8
 * 常用基本定义。
 * 历史：
 *     2020-11-12，完成声明和定长整数的定义。
 */
#if !defined(TrueMan_STDDEF)
    #define TrueMan_STDDEF
    /*########################################################################*/
    #if defined(__cplusplus)
        /*####################################################################*/
        #include <cstdint>
        /*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
        #define TrueMan_extern extern "C"
        /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
        typedef std::uint8_t   TrueMan_U8_t   ;
        typedef std::uint16_t  TrueMan_U16_t  ;
        typedef std::uint32_t  TrueMan_U32_t  ;
        typedef std::uint64_t  TrueMan_U64_t  ;
        typedef std::uintptr_t TrueMan_USIZE_t;
        /*====================================================================*/
        typedef std::int8_t   TrueMan_I8_t   ;
        typedef std::int16_t  TrueMan_I16_t  ;
        typedef std::int32_t  TrueMan_I32_t  ;
        typedef std::int64_t  TrueMan_I64_t  ;
        typedef std::intptr_t TrueMan_ISIZE_t;
        /*####################################################################*/
    #else
        /*####################################################################*/
        #include <stdint.h>
        /*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
        #define TrueMan_extern extern
        /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
        typedef uint8_t   TrueMan_U8_t   ;
        typedef uint16_t  TrueMan_U16_t  ;
        typedef uint32_t  TrueMan_U32_t  ;
        typedef uint64_t  TrueMan_U64_t  ;
        typedef uintptr_t TrueMan_USIZE_t;
        /*====================================================================*/
        typedef int8_t   TrueMan_I8_t   ;
        typedef int16_t  TrueMan_I16_t  ;
        typedef int32_t  TrueMan_I32_t  ;
        typedef int64_t  TrueMan_I64_t  ;
        typedef intptr_t TrueMan_ISIZE_t;
        /*####################################################################*/
    #endif
    /*########################################################################*/
#endif
