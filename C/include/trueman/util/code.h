/*
 * encoding=utf-8
 * 散列操作接口。
 * 历史：
 *     2020-11-13，完成BASE16编解码操作。
 *     2020-11-20，完成BASE64编解码操作。
 *     2020-11-20，完成UNICODE编解码操作。
 *     2020-11-21，完成无符号整数编解码操作。
 */
#if !defined(TrueMan_UTIL_CODE)
    #define TrueMan_UTIL_CODE
    /*########################################################################*/
    #include <trueman/stddef.h>
    /*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
    //1字节的数据，编码后得到2字节。
    //txt的字节数至少是len乘以二，所以txt的字节数至少为：
    //    len * 2
    //用位操作加速可写为：
    //    len << 1
    //cap表示字母是否用大写形式输出。
    //返回实际写入txt的字节数。
    TrueMan_extern
    TrueMan_USIZE_t TrueMan_UTIL_CODE_BASE16_encode(
        TrueMan_U8_t    const raw[],
        TrueMan_USIZE_t       len  ,
        TrueMan_U8_t          txt[],
        TrueMan_I8_t          cap
    );
    /*------------------------------------------------------------------------*/
    //2字节的数据，解码后得到1字节。
    //len应是2的整数倍；raw的字节数至少为：
    //    len / 2
    //用位操作加速可写为：
    //    len >> 1
    //返回0或正数，表示成功，写入raw多少字节；
    //返回负数，表示失败，其相反数减1，表示写入raw多少字节。
    TrueMan_extern
    TrueMan_ISIZE_t TrueMan_UTIL_CODE_BASE16_decode(
        TrueMan_U8_t const txt[], TrueMan_USIZE_t len, TrueMan_U8_t raw[]
    );
    /*------------------------------------------------------------------------*/
    //3字节的数据，编码后得到4字节。
    //txt的字节数至少是len乘以三分之四。
    //由于len不一定被3整除，所以txt的字节数至少为：
    //    ((len + 2) / 3) * 4
    //用位操作加速可写为：
    //    ((len + 2) / 3) << 2
    //返回实际写入txt的字节数。
    TrueMan_extern
    TrueMan_USIZE_t TrueMan_UTIL_CODE_BASE64_encode(
        TrueMan_U8_t const raw[], TrueMan_USIZE_t len, TrueMan_U8_t txt[]
    );
    /*------------------------------------------------------------------------*/
    //4字节的数据，解码后得到3字节。
    //len应是4的整数倍；raw的字节数至少为：
    //    ((len / 4) * 3) + (len % 4)
    //用位操作加速可写为：
    //    ((len >> 2) << 1) + (len >> 2) + (len & 3)
    //而实际写入raw的字节数可能更少，尤其当len不是4的整数倍时。
    //返回0或正数，表示成功，写入raw多少字节；
    //返回负数，表示失败，其相反数减1，表示写入raw多少字节。
    TrueMan_extern
    TrueMan_ISIZE_t TrueMan_UTIL_CODE_BASE64_decode(
        TrueMan_U8_t const txt[], TrueMan_USIZE_t len, TrueMan_U8_t raw[]
    );
    /*------------------------------------------------------------------------*/
    //将UCS-2字符串转成UTF-8编码。
    //每个UCS-2字符的取值范围是[0x0000, 0xFFFF]，即16位无符号整数的完整范围。
    //1个UCS-2字符最多转成3个字节的UTF-8编码。
    //len是ucs的元素个数。
    //返回实际写入utf的字节数。
    TrueMan_extern
    TrueMan_USIZE_t TrueMan_UTIL_CODE_UCS2_encode_utf8(
        TrueMan_U16_t const ucs[], TrueMan_USIZE_t len, TrueMan_U8_t utf[]
    );
    /*------------------------------------------------------------------------*/
    //从UTF-8解码得到UCS-2字符串。
    //最少1个字节、最多3个字节的UTF-8编码转成1个UCS-2字符。
    //len是utf的字节数。
    //返回0或正数，表示成功，写入ucs多少个元素；
    //返回负数，表示失败，其相反数减1，表示写入ucs多少个元素。
    TrueMan_extern
    TrueMan_ISIZE_t TrueMan_UTIL_CODE_UCS2_decode_utf8(
        TrueMan_U8_t const utf[], TrueMan_USIZE_t len, TrueMan_U16_t ucs[]
    );
    /*------------------------------------------------------------------------*/
    //将UCS-4字符串转成UTF-8编码。
    //每个UCS-4字符的取值范围是[0x00000000, 0x7FFFFFFF]，
    //即32位有符号整数的非负范围。
    //1个UCS-4字符最多转成6个字节的UTF-8编码。
    //len是ucs的元素个数。
    //返回实际写入utf的字节数。
    TrueMan_extern
    TrueMan_USIZE_t TrueMan_UTIL_CODE_UCS4_encode_utf8(
        TrueMan_U32_t const ucs[], TrueMan_USIZE_t len, TrueMan_U8_t utf[]
    );
    /*------------------------------------------------------------------------*/
    //从UTF-8解码得到UCS-4字符串。
    //最少1个字节、最多6个字节的UTF-8编码转成1个UCS-4字符。
    //len是utf的字节数。
    //返回0或正数，表示成功，写入ucs多少个元素；
    //返回负数，表示失败，其相反数减1，表示写入ucs多少个元素。
    TrueMan_extern
    TrueMan_ISIZE_t TrueMan_UTIL_CODE_UCS4_decode_utf8(
        TrueMan_U8_t const utf[], TrueMan_USIZE_t len, TrueMan_U32_t ucs[]
    );
    /*------------------------------------------------------------------------*/
    //以大端序解码整数。
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_CODE_UINTX_decode_be(
        TrueMan_U8_t const pos[], TrueMan_USIZE_t len
    );
    /*------------------------------------------------------------------------*/
    //以小端序解码整数。
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_CODE_UINTX_decode_le(
        TrueMan_U8_t const pos[], TrueMan_USIZE_t len
    );
    /*------------------------------------------------------------------------*/
    //以大端序编码整数。
    TrueMan_extern
    void TrueMan_UTIL_CODE_UINTX_encode_be(
        TrueMan_U8_t pos[], TrueMan_USIZE_t len, TrueMan_U64_t val
    );
    /*------------------------------------------------------------------------*/
    //以小端序编码整数。
    TrueMan_extern
    void TrueMan_UTIL_CODE_UINTX_encode_le(
        TrueMan_U8_t pos[], TrueMan_USIZE_t len, TrueMan_U64_t val
    );
    /*########################################################################*/
    #if defined(__cplusplus)
        /*####################################################################*/
        namespace TrueMan { namespace util { namespace code {
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace base16 {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline
                std::uintptr_t encode(
                    std::uint8_t   const raw[],
                    std::uintptr_t       len  ,
                    std::uint8_t         txt[],
                    bool                 cap
                ) {
                    return(TrueMan_UTIL_CODE_BASE16_encode(
                        raw, len, txt, static_cast<std::int8_t>(cap)
                    ));
                }
                /*------------------------------------------------------------*/
                static inline
                std::intptr_t decode(
                    std::uint8_t   const txt[],
                    std::uintptr_t       len  ,
                    std::uint8_t         raw[]
                ) {
                    return(TrueMan_UTIL_CODE_BASE16_decode(txt, len, raw));
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace base64 {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline
                std::uintptr_t encode(
                    std::uint8_t   const raw[],
                    std::uintptr_t       len  ,
                    std::uint8_t         txt[]
                ) {
                    return(TrueMan_UTIL_CODE_BASE64_encode(raw, len, txt));
                }
                /*------------------------------------------------------------*/
                static inline
                std::intptr_t decode(
                    std::uint8_t   const txt[],
                    std::uintptr_t       len  ,
                    std::uint8_t         raw[]
                ) {
                    return(TrueMan_UTIL_CODE_BASE64_decode(txt, len, raw));
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace ucs2 {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline
                std::uintptr_t encode_utf8(
                    std::uint16_t  const ucs[],
                    std::uintptr_t       len  ,
                    std::uint8_t         utf[]
                ) {
                    return(TrueMan_UTIL_CODE_UCS2_encode_utf8(ucs, len, utf));
                }
                /*------------------------------------------------------------*/
                static inline
                std::intptr_t decode_utf8(
                    std::uint8_t   const utf[],
                    std::uintptr_t       len  ,
                    std::uint16_t        ucs[]
                ) {
                    return(TrueMan_UTIL_CODE_UCS2_decode_utf8(utf, len, ucs));
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace ucs4 {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline
                std::uintptr_t encode_utf8(
                    std::uint32_t  const ucs[],
                    std::uintptr_t       len  ,
                    std::uint8_t         utf[]
                ) {
                    return(TrueMan_UTIL_CODE_UCS4_encode_utf8(ucs, len, utf));
                }
                /*------------------------------------------------------------*/
                static inline
                std::intptr_t decode_utf8(
                    std::uint8_t   const utf[],
                    std::uintptr_t       len  ,
                    std::uint32_t        ucs[]
                ) {
                    return(TrueMan_UTIL_CODE_UCS4_decode_utf8(utf, len, ucs));
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace uintx {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline
                std::uint64_t decode_be(
                    std::uint8_t const pos[], std::uintptr_t len
                ) {
                    return(TrueMan_UTIL_CODE_UINTX_decode_be(pos, len));
                }
                /*------------------------------------------------------------*/
                static inline
                std::uint64_t decode_le(
                    std::uint8_t const pos[], std::uintptr_t len
                ) {
                    return(TrueMan_UTIL_CODE_UINTX_decode_le(pos, len));
                }
                /*------------------------------------------------------------*/
                static inline
                void encode_be(
                    std::uint8_t pos[], std::uintptr_t len, std::uint64_t val
                ) {
                    TrueMan_UTIL_CODE_UINTX_encode_be(pos, len, val);
                }
                /*------------------------------------------------------------*/
                static inline
                void encode_le(
                    std::uint8_t pos[], std::uintptr_t len, std::uint64_t val
                ) {
                    TrueMan_UTIL_CODE_UINTX_encode_le(pos, len, val);
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
        } } }
        /*####################################################################*/
    #endif
    /*########################################################################*/
#endif
