/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Identify Bison output, and Bison version.  */
#define YYBISON 30802

/* Bison version string.  */
#define YYBISON_VERSION "3.8.2"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 0

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1


/* Substitute the variable and function names.  */
#define yyparse         parser_parse
#define yylex           parser_lex
#define yyerror         parser_error
#define yydebug         parser_debug
#define yynerrs         parser_nerrs
#define yylval          parser_lval
#define yychar          parser_char

/* First part of user prologue.  */
#line 1 "/home/moebius/reactos/sdk/tools/widl/parser.y"

/*
 * IDL Compiler
 *
 * Copyright 2002 Ove Kaaven
 * Copyright 2006-2008 Robert Shearman
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301, USA
 */

#include "config.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <assert.h>
#include <ctype.h>
#include <string.h>

#include "widl.h"
#include "utils.h"
#include "parser.h"
#include "header.h"
#include "typelib.h"
#include "typegen.h"
#include "expr.h"
#include "typetree.h"

static unsigned char pointer_default = FC_UP;

typedef struct list typelist_t;
struct typenode {
  type_t *type;
  struct list entry;
};

struct _import_t
{
  char *name;
  int import_performed;
};

typedef struct _decl_spec_t
{
  type_t *type;
  attr_list_t *attrs;
  enum storage_class stgclass;
} decl_spec_t;

typelist_t incomplete_types = LIST_INIT(incomplete_types);

static void fix_incomplete(void);
static void fix_incomplete_types(type_t *complete_type);

static str_list_t *append_str(str_list_t *list, char *str);
static attr_list_t *append_attr(attr_list_t *list, attr_t *attr);
static attr_list_t *append_attr_list(attr_list_t *new_list, attr_list_t *old_list);
static decl_spec_t *make_decl_spec(type_t *type, decl_spec_t *left, decl_spec_t *right, attr_t *attr, enum storage_class stgclass);
static attr_t *make_attr(enum attr_type type);
static attr_t *make_attrv(enum attr_type type, unsigned int val);
static attr_t *make_attrp(enum attr_type type, void *val);
static expr_list_t *append_expr(expr_list_t *list, expr_t *expr);
static type_t *append_array(type_t *chain, expr_t *expr);
static var_t *declare_var(attr_list_t *attrs, decl_spec_t *decl_spec, const declarator_t *decl, int top);
static var_list_t *set_var_types(attr_list_t *attrs, decl_spec_t *decl_spec, declarator_list_t *decls);
static ifref_list_t *append_ifref(ifref_list_t *list, ifref_t *iface);
static ifref_t *make_ifref(type_t *iface);
static var_list_t *append_var_list(var_list_t *list, var_list_t *vars);
static declarator_list_t *append_declarator(declarator_list_t *list, declarator_t *p);
static declarator_t *make_declarator(var_t *var);
static type_t *make_safearray(type_t *type);
static typelib_t *make_library(const char *name, const attr_list_t *attrs);
static type_t *append_chain_type(type_t *chain, type_t *type);
static warning_list_t *append_warning(warning_list_t *, int);

static type_t *reg_typedefs(decl_spec_t *decl_spec, var_list_t *names, attr_list_t *attrs);
static type_t *find_type_or_error(const char *name, int t);
static type_t *find_type_or_error2(char *name, int t);

static var_t *reg_const(var_t *var);

static void push_namespace(const char *name);
static void pop_namespace(const char *name);

static char *gen_name(void);
static void check_arg_attrs(const var_t *arg);
static void check_statements(const statement_list_t *stmts, int is_inside_library);
static void check_all_user_types(const statement_list_t *stmts);
static attr_list_t *check_iface_attrs(const char *name, attr_list_t *attrs);
static attr_list_t *check_function_attrs(const char *name, attr_list_t *attrs);
static attr_list_t *check_typedef_attrs(attr_list_t *attrs);
static attr_list_t *check_enum_attrs(attr_list_t *attrs);
static attr_list_t *check_struct_attrs(attr_list_t *attrs);
static attr_list_t *check_union_attrs(attr_list_t *attrs);
static attr_list_t *check_field_attrs(const char *name, attr_list_t *attrs);
static attr_list_t *check_library_attrs(const char *name, attr_list_t *attrs);
static attr_list_t *check_dispiface_attrs(const char *name, attr_list_t *attrs);
static attr_list_t *check_module_attrs(const char *name, attr_list_t *attrs);
static attr_list_t *check_coclass_attrs(const char *name, attr_list_t *attrs);
const char *get_attr_display_name(enum attr_type type);
static void add_explicit_handle_if_necessary(const type_t *iface, var_t *func);
static void check_def(const type_t *t);

static void check_async_uuid(type_t *iface);

static statement_t *make_statement(enum statement_type type);
static statement_t *make_statement_type_decl(type_t *type);
static statement_t *make_statement_reference(type_t *type);
static statement_t *make_statement_declaration(var_t *var);
static statement_t *make_statement_library(typelib_t *typelib);
static statement_t *make_statement_pragma(const char *str);
static statement_t *make_statement_cppquote(const char *str);
static statement_t *make_statement_importlib(const char *str);
static statement_t *make_statement_module(type_t *type);
static statement_t *make_statement_typedef(var_list_t *names);
static statement_t *make_statement_import(const char *str);
static statement_list_t *append_statement(statement_list_t *list, statement_t *stmt);
static statement_list_t *append_statements(statement_list_t *, statement_list_t *);
static attr_list_t *append_attribs(attr_list_t *, attr_list_t *);

static struct namespace global_namespace = {
    NULL, NULL, LIST_INIT(global_namespace.entry), LIST_INIT(global_namespace.children)
};

static struct namespace *current_namespace = &global_namespace;

static typelib_t *current_typelib;


#line 220 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"

# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif

#include "parser.tab.h"
/* Symbol kind.  */
enum yysymbol_kind_t
{
  YYSYMBOL_YYEMPTY = -2,
  YYSYMBOL_YYEOF = 0,                      /* "end of file"  */
  YYSYMBOL_YYerror = 1,                    /* error  */
  YYSYMBOL_YYUNDEF = 2,                    /* "invalid token"  */
  YYSYMBOL_aIDENTIFIER = 3,                /* aIDENTIFIER  */
  YYSYMBOL_aPRAGMA = 4,                    /* aPRAGMA  */
  YYSYMBOL_aKNOWNTYPE = 5,                 /* aKNOWNTYPE  */
  YYSYMBOL_aNUM = 6,                       /* aNUM  */
  YYSYMBOL_aHEXNUM = 7,                    /* aHEXNUM  */
  YYSYMBOL_aDOUBLE = 8,                    /* aDOUBLE  */
  YYSYMBOL_aSTRING = 9,                    /* aSTRING  */
  YYSYMBOL_aWSTRING = 10,                  /* aWSTRING  */
  YYSYMBOL_aSQSTRING = 11,                 /* aSQSTRING  */
  YYSYMBOL_aUUID = 12,                     /* aUUID  */
  YYSYMBOL_aEOF = 13,                      /* aEOF  */
  YYSYMBOL_aACF = 14,                      /* aACF  */
  YYSYMBOL_SHL = 15,                       /* SHL  */
  YYSYMBOL_SHR = 16,                       /* SHR  */
  YYSYMBOL_MEMBERPTR = 17,                 /* MEMBERPTR  */
  YYSYMBOL_EQUALITY = 18,                  /* EQUALITY  */
  YYSYMBOL_INEQUALITY = 19,                /* INEQUALITY  */
  YYSYMBOL_GREATEREQUAL = 20,              /* GREATEREQUAL  */
  YYSYMBOL_LESSEQUAL = 21,                 /* LESSEQUAL  */
  YYSYMBOL_LOGICALOR = 22,                 /* LOGICALOR  */
  YYSYMBOL_LOGICALAND = 23,                /* LOGICALAND  */
  YYSYMBOL_ELLIPSIS = 24,                  /* ELLIPSIS  */
  YYSYMBOL_tAGGREGATABLE = 25,             /* tAGGREGATABLE  */
  YYSYMBOL_tALLNODES = 26,                 /* tALLNODES  */
  YYSYMBOL_tALLOCATE = 27,                 /* tALLOCATE  */
  YYSYMBOL_tANNOTATION = 28,               /* tANNOTATION  */
  YYSYMBOL_tAPPOBJECT = 29,                /* tAPPOBJECT  */
  YYSYMBOL_tASYNC = 30,                    /* tASYNC  */
  YYSYMBOL_tASYNCUUID = 31,                /* tASYNCUUID  */
  YYSYMBOL_tAUTOHANDLE = 32,               /* tAUTOHANDLE  */
  YYSYMBOL_tBINDABLE = 33,                 /* tBINDABLE  */
  YYSYMBOL_tBOOLEAN = 34,                  /* tBOOLEAN  */
  YYSYMBOL_tBROADCAST = 35,                /* tBROADCAST  */
  YYSYMBOL_tBYTE = 36,                     /* tBYTE  */
  YYSYMBOL_tBYTECOUNT = 37,                /* tBYTECOUNT  */
  YYSYMBOL_tCALLAS = 38,                   /* tCALLAS  */
  YYSYMBOL_tCALLBACK = 39,                 /* tCALLBACK  */
  YYSYMBOL_tCASE = 40,                     /* tCASE  */
  YYSYMBOL_tCDECL = 41,                    /* tCDECL  */
  YYSYMBOL_tCHAR = 42,                     /* tCHAR  */
  YYSYMBOL_tCOCLASS = 43,                  /* tCOCLASS  */
  YYSYMBOL_tCODE = 44,                     /* tCODE  */
  YYSYMBOL_tCOMMSTATUS = 45,               /* tCOMMSTATUS  */
  YYSYMBOL_tCONST = 46,                    /* tCONST  */
  YYSYMBOL_tCONTEXTHANDLE = 47,            /* tCONTEXTHANDLE  */
  YYSYMBOL_tCONTEXTHANDLENOSERIALIZE = 48, /* tCONTEXTHANDLENOSERIALIZE  */
  YYSYMBOL_tCONTEXTHANDLESERIALIZE = 49,   /* tCONTEXTHANDLESERIALIZE  */
  YYSYMBOL_tCONTROL = 50,                  /* tCONTROL  */
  YYSYMBOL_tCPPQUOTE = 51,                 /* tCPPQUOTE  */
  YYSYMBOL_tDECODE = 52,                   /* tDECODE  */
  YYSYMBOL_tDEFAULT = 53,                  /* tDEFAULT  */
  YYSYMBOL_tDEFAULTBIND = 54,              /* tDEFAULTBIND  */
  YYSYMBOL_tDEFAULTCOLLELEM = 55,          /* tDEFAULTCOLLELEM  */
  YYSYMBOL_tDEFAULTVALUE = 56,             /* tDEFAULTVALUE  */
  YYSYMBOL_tDEFAULTVTABLE = 57,            /* tDEFAULTVTABLE  */
  YYSYMBOL_tDISABLECONSISTENCYCHECK = 58,  /* tDISABLECONSISTENCYCHECK  */
  YYSYMBOL_tDISPLAYBIND = 59,              /* tDISPLAYBIND  */
  YYSYMBOL_tDISPINTERFACE = 60,            /* tDISPINTERFACE  */
  YYSYMBOL_tDLLNAME = 61,                  /* tDLLNAME  */
  YYSYMBOL_tDONTFREE = 62,                 /* tDONTFREE  */
  YYSYMBOL_tDOUBLE = 63,                   /* tDOUBLE  */
  YYSYMBOL_tDUAL = 64,                     /* tDUAL  */
  YYSYMBOL_tENABLEALLOCATE = 65,           /* tENABLEALLOCATE  */
  YYSYMBOL_tENCODE = 66,                   /* tENCODE  */
  YYSYMBOL_tENDPOINT = 67,                 /* tENDPOINT  */
  YYSYMBOL_tENTRY = 68,                    /* tENTRY  */
  YYSYMBOL_tENUM = 69,                     /* tENUM  */
  YYSYMBOL_tERRORSTATUST = 70,             /* tERRORSTATUST  */
  YYSYMBOL_tEXPLICITHANDLE = 71,           /* tEXPLICITHANDLE  */
  YYSYMBOL_tEXTERN = 72,                   /* tEXTERN  */
  YYSYMBOL_tFALSE = 73,                    /* tFALSE  */
  YYSYMBOL_tFASTCALL = 74,                 /* tFASTCALL  */
  YYSYMBOL_tFAULTSTATUS = 75,              /* tFAULTSTATUS  */
  YYSYMBOL_tFLOAT = 76,                    /* tFLOAT  */
  YYSYMBOL_tFORCEALLOCATE = 77,            /* tFORCEALLOCATE  */
  YYSYMBOL_tHANDLE = 78,                   /* tHANDLE  */
  YYSYMBOL_tHANDLET = 79,                  /* tHANDLET  */
  YYSYMBOL_tHELPCONTEXT = 80,              /* tHELPCONTEXT  */
  YYSYMBOL_tHELPFILE = 81,                 /* tHELPFILE  */
  YYSYMBOL_tHELPSTRING = 82,               /* tHELPSTRING  */
  YYSYMBOL_tHELPSTRINGCONTEXT = 83,        /* tHELPSTRINGCONTEXT  */
  YYSYMBOL_tHELPSTRINGDLL = 84,            /* tHELPSTRINGDLL  */
  YYSYMBOL_tHIDDEN = 85,                   /* tHIDDEN  */
  YYSYMBOL_tHYPER = 86,                    /* tHYPER  */
  YYSYMBOL_tID = 87,                       /* tID  */
  YYSYMBOL_tIDEMPOTENT = 88,               /* tIDEMPOTENT  */
  YYSYMBOL_tIGNORE = 89,                   /* tIGNORE  */
  YYSYMBOL_tIIDIS = 90,                    /* tIIDIS  */
  YYSYMBOL_tIMMEDIATEBIND = 91,            /* tIMMEDIATEBIND  */
  YYSYMBOL_tIMPLICITHANDLE = 92,           /* tIMPLICITHANDLE  */
  YYSYMBOL_tIMPORT = 93,                   /* tIMPORT  */
  YYSYMBOL_tIMPORTLIB = 94,                /* tIMPORTLIB  */
  YYSYMBOL_tIN = 95,                       /* tIN  */
  YYSYMBOL_tIN_LINE = 96,                  /* tIN_LINE  */
  YYSYMBOL_tINLINE = 97,                   /* tINLINE  */
  YYSYMBOL_tINPUTSYNC = 98,                /* tINPUTSYNC  */
  YYSYMBOL_tINT = 99,                      /* tINT  */
  YYSYMBOL_tINT32 = 100,                   /* tINT32  */
  YYSYMBOL_tINT3264 = 101,                 /* tINT3264  */
  YYSYMBOL_tINT64 = 102,                   /* tINT64  */
  YYSYMBOL_tINTERFACE = 103,               /* tINTERFACE  */
  YYSYMBOL_tLCID = 104,                    /* tLCID  */
  YYSYMBOL_tLENGTHIS = 105,                /* tLENGTHIS  */
  YYSYMBOL_tLIBRARY = 106,                 /* tLIBRARY  */
  YYSYMBOL_tLICENSED = 107,                /* tLICENSED  */
  YYSYMBOL_tLOCAL = 108,                   /* tLOCAL  */
  YYSYMBOL_tLONG = 109,                    /* tLONG  */
  YYSYMBOL_tMAYBE = 110,                   /* tMAYBE  */
  YYSYMBOL_tMESSAGE = 111,                 /* tMESSAGE  */
  YYSYMBOL_tMETHODS = 112,                 /* tMETHODS  */
  YYSYMBOL_tMODULE = 113,                  /* tMODULE  */
  YYSYMBOL_tNAMESPACE = 114,               /* tNAMESPACE  */
  YYSYMBOL_tNOCODE = 115,                  /* tNOCODE  */
  YYSYMBOL_tNONBROWSABLE = 116,            /* tNONBROWSABLE  */
  YYSYMBOL_tNONCREATABLE = 117,            /* tNONCREATABLE  */
  YYSYMBOL_tNONEXTENSIBLE = 118,           /* tNONEXTENSIBLE  */
  YYSYMBOL_tNOTIFY = 119,                  /* tNOTIFY  */
  YYSYMBOL_tNOTIFYFLAG = 120,              /* tNOTIFYFLAG  */
  YYSYMBOL_tNULL = 121,                    /* tNULL  */
  YYSYMBOL_tOBJECT = 122,                  /* tOBJECT  */
  YYSYMBOL_tODL = 123,                     /* tODL  */
  YYSYMBOL_tOLEAUTOMATION = 124,           /* tOLEAUTOMATION  */
  YYSYMBOL_tOPTIMIZE = 125,                /* tOPTIMIZE  */
  YYSYMBOL_tOPTIONAL = 126,                /* tOPTIONAL  */
  YYSYMBOL_tOUT = 127,                     /* tOUT  */
  YYSYMBOL_tPARTIALIGNORE = 128,           /* tPARTIALIGNORE  */
  YYSYMBOL_tPASCAL = 129,                  /* tPASCAL  */
  YYSYMBOL_tPOINTERDEFAULT = 130,          /* tPOINTERDEFAULT  */
  YYSYMBOL_tPRAGMA_WARNING = 131,          /* tPRAGMA_WARNING  */
  YYSYMBOL_tPROGID = 132,                  /* tPROGID  */
  YYSYMBOL_tPROPERTIES = 133,              /* tPROPERTIES  */
  YYSYMBOL_tPROPGET = 134,                 /* tPROPGET  */
  YYSYMBOL_tPROPPUT = 135,                 /* tPROPPUT  */
  YYSYMBOL_tPROPPUTREF = 136,              /* tPROPPUTREF  */
  YYSYMBOL_tPROXY = 137,                   /* tPROXY  */
  YYSYMBOL_tPTR = 138,                     /* tPTR  */
  YYSYMBOL_tPUBLIC = 139,                  /* tPUBLIC  */
  YYSYMBOL_tRANGE = 140,                   /* tRANGE  */
  YYSYMBOL_tREADONLY = 141,                /* tREADONLY  */
  YYSYMBOL_tREF = 142,                     /* tREF  */
  YYSYMBOL_tREGISTER = 143,                /* tREGISTER  */
  YYSYMBOL_tREPRESENTAS = 144,             /* tREPRESENTAS  */
  YYSYMBOL_tREQUESTEDIT = 145,             /* tREQUESTEDIT  */
  YYSYMBOL_tRESTRICTED = 146,              /* tRESTRICTED  */
  YYSYMBOL_tRETVAL = 147,                  /* tRETVAL  */
  YYSYMBOL_tSAFEARRAY = 148,               /* tSAFEARRAY  */
  YYSYMBOL_tSHORT = 149,                   /* tSHORT  */
  YYSYMBOL_tSIGNED = 150,                  /* tSIGNED  */
  YYSYMBOL_tSINGLENODE = 151,              /* tSINGLENODE  */
  YYSYMBOL_tSIZEIS = 152,                  /* tSIZEIS  */
  YYSYMBOL_tSIZEOF = 153,                  /* tSIZEOF  */
  YYSYMBOL_tSMALL = 154,                   /* tSMALL  */
  YYSYMBOL_tSOURCE = 155,                  /* tSOURCE  */
  YYSYMBOL_tSTATIC = 156,                  /* tSTATIC  */
  YYSYMBOL_tSTDCALL = 157,                 /* tSTDCALL  */
  YYSYMBOL_tSTRICTCONTEXTHANDLE = 158,     /* tSTRICTCONTEXTHANDLE  */
  YYSYMBOL_tSTRING = 159,                  /* tSTRING  */
  YYSYMBOL_tSTRUCT = 160,                  /* tSTRUCT  */
  YYSYMBOL_tSWITCH = 161,                  /* tSWITCH  */
  YYSYMBOL_tSWITCHIS = 162,                /* tSWITCHIS  */
  YYSYMBOL_tSWITCHTYPE = 163,              /* tSWITCHTYPE  */
  YYSYMBOL_tTHREADING = 164,               /* tTHREADING  */
  YYSYMBOL_tTRANSMITAS = 165,              /* tTRANSMITAS  */
  YYSYMBOL_tTRUE = 166,                    /* tTRUE  */
  YYSYMBOL_tTYPEDEF = 167,                 /* tTYPEDEF  */
  YYSYMBOL_tUIDEFAULT = 168,               /* tUIDEFAULT  */
  YYSYMBOL_tUNION = 169,                   /* tUNION  */
  YYSYMBOL_tUNIQUE = 170,                  /* tUNIQUE  */
  YYSYMBOL_tUNSIGNED = 171,                /* tUNSIGNED  */
  YYSYMBOL_tUSESGETLASTERROR = 172,        /* tUSESGETLASTERROR  */
  YYSYMBOL_tUSERMARSHAL = 173,             /* tUSERMARSHAL  */
  YYSYMBOL_tUUID = 174,                    /* tUUID  */
  YYSYMBOL_tV1ENUM = 175,                  /* tV1ENUM  */
  YYSYMBOL_tVARARG = 176,                  /* tVARARG  */
  YYSYMBOL_tVERSION = 177,                 /* tVERSION  */
  YYSYMBOL_tVIPROGID = 178,                /* tVIPROGID  */
  YYSYMBOL_tVOID = 179,                    /* tVOID  */
  YYSYMBOL_tWCHAR = 180,                   /* tWCHAR  */
  YYSYMBOL_tWIREMARSHAL = 181,             /* tWIREMARSHAL  */
  YYSYMBOL_tAPARTMENT = 182,               /* tAPARTMENT  */
  YYSYMBOL_tNEUTRAL = 183,                 /* tNEUTRAL  */
  YYSYMBOL_tSINGLE = 184,                  /* tSINGLE  */
  YYSYMBOL_tFREE = 185,                    /* tFREE  */
  YYSYMBOL_tBOTH = 186,                    /* tBOTH  */
  YYSYMBOL_187_ = 187,                     /* ','  */
  YYSYMBOL_188_ = 188,                     /* '?'  */
  YYSYMBOL_189_ = 189,                     /* ':'  */
  YYSYMBOL_190_ = 190,                     /* '|'  */
  YYSYMBOL_191_ = 191,                     /* '^'  */
  YYSYMBOL_192_ = 192,                     /* '&'  */
  YYSYMBOL_193_ = 193,                     /* '<'  */
  YYSYMBOL_194_ = 194,                     /* '>'  */
  YYSYMBOL_195_ = 195,                     /* '-'  */
  YYSYMBOL_196_ = 196,                     /* '+'  */
  YYSYMBOL_197_ = 197,                     /* '*'  */
  YYSYMBOL_198_ = 198,                     /* '/'  */
  YYSYMBOL_199_ = 199,                     /* '%'  */
  YYSYMBOL_200_ = 200,                     /* '!'  */
  YYSYMBOL_201_ = 201,                     /* '~'  */
  YYSYMBOL_CAST = 202,                     /* CAST  */
  YYSYMBOL_PPTR = 203,                     /* PPTR  */
  YYSYMBOL_POS = 204,                      /* POS  */
  YYSYMBOL_NEG = 205,                      /* NEG  */
  YYSYMBOL_ADDRESSOF = 206,                /* ADDRESSOF  */
  YYSYMBOL_207_ = 207,                     /* '.'  */
  YYSYMBOL_208_ = 208,                     /* '['  */
  YYSYMBOL_209_ = 209,                     /* ']'  */
  YYSYMBOL_210_ = 210,                     /* '{'  */
  YYSYMBOL_211_ = 211,                     /* '}'  */
  YYSYMBOL_212_ = 212,                     /* ';'  */
  YYSYMBOL_213_ = 213,                     /* '('  */
  YYSYMBOL_214_ = 214,                     /* ')'  */
  YYSYMBOL_215_ = 215,                     /* '='  */
  YYSYMBOL_YYACCEPT = 216,                 /* $accept  */
  YYSYMBOL_input = 217,                    /* input  */
  YYSYMBOL_m_acf = 218,                    /* m_acf  */
  YYSYMBOL_gbl_statements = 219,           /* gbl_statements  */
  YYSYMBOL_220_1 = 220,                    /* $@1  */
  YYSYMBOL_imp_statements = 221,           /* imp_statements  */
  YYSYMBOL_222_2 = 222,                    /* $@2  */
  YYSYMBOL_int_statements = 223,           /* int_statements  */
  YYSYMBOL_semicolon_opt = 224,            /* semicolon_opt  */
  YYSYMBOL_statement = 225,                /* statement  */
  YYSYMBOL_pragma_warning = 226,           /* pragma_warning  */
  YYSYMBOL_warnings = 227,                 /* warnings  */
  YYSYMBOL_typedecl = 228,                 /* typedecl  */
  YYSYMBOL_cppquote = 229,                 /* cppquote  */
  YYSYMBOL_import_start = 230,             /* import_start  */
  YYSYMBOL_import = 231,                   /* import  */
  YYSYMBOL_importlib = 232,                /* importlib  */
  YYSYMBOL_libraryhdr = 233,               /* libraryhdr  */
  YYSYMBOL_library_start = 234,            /* library_start  */
  YYSYMBOL_librarydef = 235,               /* librarydef  */
  YYSYMBOL_m_args = 236,                   /* m_args  */
  YYSYMBOL_arg_list = 237,                 /* arg_list  */
  YYSYMBOL_args = 238,                     /* args  */
  YYSYMBOL_arg = 239,                      /* arg  */
  YYSYMBOL_array = 240,                    /* array  */
  YYSYMBOL_m_attributes = 241,             /* m_attributes  */
  YYSYMBOL_attributes = 242,               /* attributes  */
  YYSYMBOL_attrib_list = 243,              /* attrib_list  */
  YYSYMBOL_str_list = 244,                 /* str_list  */
  YYSYMBOL_attribute = 245,                /* attribute  */
  YYSYMBOL_uuid_string = 246,              /* uuid_string  */
  YYSYMBOL_callconv = 247,                 /* callconv  */
  YYSYMBOL_cases = 248,                    /* cases  */
  YYSYMBOL_case = 249,                     /* case  */
  YYSYMBOL_enums = 250,                    /* enums  */
  YYSYMBOL_enum_list = 251,                /* enum_list  */
  YYSYMBOL_enum = 252,                     /* enum  */
  YYSYMBOL_enumdef = 253,                  /* enumdef  */
  YYSYMBOL_m_exprs = 254,                  /* m_exprs  */
  YYSYMBOL_m_expr = 255,                   /* m_expr  */
  YYSYMBOL_expr = 256,                     /* expr  */
  YYSYMBOL_expr_list_int_const = 257,      /* expr_list_int_const  */
  YYSYMBOL_expr_int_const = 258,           /* expr_int_const  */
  YYSYMBOL_expr_const = 259,               /* expr_const  */
  YYSYMBOL_fields = 260,                   /* fields  */
  YYSYMBOL_field = 261,                    /* field  */
  YYSYMBOL_ne_union_field = 262,           /* ne_union_field  */
  YYSYMBOL_ne_union_fields = 263,          /* ne_union_fields  */
  YYSYMBOL_union_field = 264,              /* union_field  */
  YYSYMBOL_s_field = 265,                  /* s_field  */
  YYSYMBOL_funcdef = 266,                  /* funcdef  */
  YYSYMBOL_declaration = 267,              /* declaration  */
  YYSYMBOL_m_ident = 268,                  /* m_ident  */
  YYSYMBOL_t_ident = 269,                  /* t_ident  */
  YYSYMBOL_ident = 270,                    /* ident  */
  YYSYMBOL_base_type = 271,                /* base_type  */
  YYSYMBOL_m_int = 272,                    /* m_int  */
  YYSYMBOL_int_std = 273,                  /* int_std  */
  YYSYMBOL_coclass = 274,                  /* coclass  */
  YYSYMBOL_coclasshdr = 275,               /* coclasshdr  */
  YYSYMBOL_coclassdef = 276,               /* coclassdef  */
  YYSYMBOL_namespacedef = 277,             /* namespacedef  */
  YYSYMBOL_coclass_ints = 278,             /* coclass_ints  */
  YYSYMBOL_coclass_int = 279,              /* coclass_int  */
  YYSYMBOL_dispinterface = 280,            /* dispinterface  */
  YYSYMBOL_dispinterfacehdr = 281,         /* dispinterfacehdr  */
  YYSYMBOL_dispint_props = 282,            /* dispint_props  */
  YYSYMBOL_dispint_meths = 283,            /* dispint_meths  */
  YYSYMBOL_dispinterfacedef = 284,         /* dispinterfacedef  */
  YYSYMBOL_inherit = 285,                  /* inherit  */
  YYSYMBOL_interface = 286,                /* interface  */
  YYSYMBOL_interfacehdr = 287,             /* interfacehdr  */
  YYSYMBOL_interfacedef = 288,             /* interfacedef  */
  YYSYMBOL_interfacedec = 289,             /* interfacedec  */
  YYSYMBOL_module = 290,                   /* module  */
  YYSYMBOL_modulehdr = 291,                /* modulehdr  */
  YYSYMBOL_moduledef = 292,                /* moduledef  */
  YYSYMBOL_storage_cls_spec = 293,         /* storage_cls_spec  */
  YYSYMBOL_function_specifier = 294,       /* function_specifier  */
  YYSYMBOL_type_qualifier = 295,           /* type_qualifier  */
  YYSYMBOL_m_type_qual_list = 296,         /* m_type_qual_list  */
  YYSYMBOL_decl_spec = 297,                /* decl_spec  */
  YYSYMBOL_m_decl_spec_no_type = 298,      /* m_decl_spec_no_type  */
  YYSYMBOL_decl_spec_no_type = 299,        /* decl_spec_no_type  */
  YYSYMBOL_declarator = 300,               /* declarator  */
  YYSYMBOL_direct_declarator = 301,        /* direct_declarator  */
  YYSYMBOL_abstract_declarator = 302,      /* abstract_declarator  */
  YYSYMBOL_abstract_declarator_no_direct = 303, /* abstract_declarator_no_direct  */
  YYSYMBOL_m_abstract_declarator = 304,    /* m_abstract_declarator  */
  YYSYMBOL_abstract_direct_declarator = 305, /* abstract_direct_declarator  */
  YYSYMBOL_any_declarator = 306,           /* any_declarator  */
  YYSYMBOL_any_declarator_no_direct = 307, /* any_declarator_no_direct  */
  YYSYMBOL_m_any_declarator = 308,         /* m_any_declarator  */
  YYSYMBOL_any_direct_declarator = 309,    /* any_direct_declarator  */
  YYSYMBOL_declarator_list = 310,          /* declarator_list  */
  YYSYMBOL_m_bitfield = 311,               /* m_bitfield  */
  YYSYMBOL_struct_declarator = 312,        /* struct_declarator  */
  YYSYMBOL_struct_declarator_list = 313,   /* struct_declarator_list  */
  YYSYMBOL_init_declarator = 314,          /* init_declarator  */
  YYSYMBOL_threading_type = 315,           /* threading_type  */
  YYSYMBOL_pointer_type = 316,             /* pointer_type  */
  YYSYMBOL_structdef = 317,                /* structdef  */
  YYSYMBOL_type = 318,                     /* type  */
  YYSYMBOL_typedef = 319,                  /* typedef  */
  YYSYMBOL_uniondef = 320,                 /* uniondef  */
  YYSYMBOL_version = 321,                  /* version  */
  YYSYMBOL_acf_statements = 322,           /* acf_statements  */
  YYSYMBOL_acf_int_statements = 323,       /* acf_int_statements  */
  YYSYMBOL_acf_int_statement = 324,        /* acf_int_statement  */
  YYSYMBOL_acf_interface = 325,            /* acf_interface  */
  YYSYMBOL_acf_attributes = 326,           /* acf_attributes  */
  YYSYMBOL_acf_attribute_list = 327,       /* acf_attribute_list  */
  YYSYMBOL_acf_attribute = 328,            /* acf_attribute  */
  YYSYMBOL_allocate_option_list = 329,     /* allocate_option_list  */
  YYSYMBOL_allocate_option = 330           /* allocate_option  */
};
typedef enum yysymbol_kind_t yysymbol_kind_t;




#ifdef short
# undef short
#endif

/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */

#ifndef __PTRDIFF_MAX__
# include <limits.h> /* INFRINGES ON USER NAME SPACE */
# if defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stdint.h> /* INFRINGES ON USER NAME SPACE */
#  define YY_STDINT_H
# endif
#endif

/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */

#ifdef __INT_LEAST8_MAX__
typedef __INT_LEAST8_TYPE__ yytype_int8;
#elif defined YY_STDINT_H
typedef int_least8_t yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef __INT_LEAST16_MAX__
typedef __INT_LEAST16_TYPE__ yytype_int16;
#elif defined YY_STDINT_H
typedef int_least16_t yytype_int16;
#else
typedef short yytype_int16;
#endif

/* Work around bug in HP-UX 11.23, which defines these macros
   incorrectly for preprocessor constants.  This workaround can likely
   be removed in 2023, as HPE has promised support for HP-UX 11.23
   (aka HP-UX 11i v2) only through the end of 2022; see Table 2 of
   <https://h20195.www2.hpe.com/V2/getpdf.aspx/4AA4-7673ENW.pdf>.  */
#ifdef __hpux
# undef UINT_LEAST8_MAX
# undef UINT_LEAST16_MAX
# define UINT_LEAST8_MAX 255
# define UINT_LEAST16_MAX 65535
#endif

#if defined __UINT_LEAST8_MAX__ && __UINT_LEAST8_MAX__ <= __INT_MAX__
typedef __UINT_LEAST8_TYPE__ yytype_uint8;
#elif (!defined __UINT_LEAST8_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST8_MAX <= INT_MAX)
typedef uint_least8_t yytype_uint8;
#elif !defined __UINT_LEAST8_MAX__ && UCHAR_MAX <= INT_MAX
typedef unsigned char yytype_uint8;
#else
typedef short yytype_uint8;
#endif

#if defined __UINT_LEAST16_MAX__ && __UINT_LEAST16_MAX__ <= __INT_MAX__
typedef __UINT_LEAST16_TYPE__ yytype_uint16;
#elif (!defined __UINT_LEAST16_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST16_MAX <= INT_MAX)
typedef uint_least16_t yytype_uint16;
#elif !defined __UINT_LEAST16_MAX__ && USHRT_MAX <= INT_MAX
typedef unsigned short yytype_uint16;
#else
typedef int yytype_uint16;
#endif

#ifndef YYPTRDIFF_T
# if defined __PTRDIFF_TYPE__ && defined __PTRDIFF_MAX__
#  define YYPTRDIFF_T __PTRDIFF_TYPE__
#  define YYPTRDIFF_MAXIMUM __PTRDIFF_MAX__
# elif defined PTRDIFF_MAX
#  ifndef ptrdiff_t
#   include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  endif
#  define YYPTRDIFF_T ptrdiff_t
#  define YYPTRDIFF_MAXIMUM PTRDIFF_MAX
# else
#  define YYPTRDIFF_T long
#  define YYPTRDIFF_MAXIMUM LONG_MAX
# endif
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned
# endif
#endif

#define YYSIZE_MAXIMUM                                  \
  YY_CAST (YYPTRDIFF_T,                                 \
           (YYPTRDIFF_MAXIMUM < YY_CAST (YYSIZE_T, -1)  \
            ? YYPTRDIFF_MAXIMUM                         \
            : YY_CAST (YYSIZE_T, -1)))

#define YYSIZEOF(X) YY_CAST (YYPTRDIFF_T, sizeof (X))


/* Stored state numbers (used for stacks). */
typedef yytype_int16 yy_state_t;

/* State numbers in computations.  */
typedef int yy_state_fast_t;

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif


#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YY_USE(E) ((void) (E))
#else
# define YY_USE(E) /* empty */
#endif

/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
#if defined __GNUC__ && ! defined __ICC && 406 <= __GNUC__ * 100 + __GNUC_MINOR__
# if __GNUC__ * 100 + __GNUC_MINOR__ < 407
#  define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                           \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")
# else
#  define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                           \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# endif
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#define YY_ASSERT(E) ((void) (0 && (E)))

#if 1

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* 1 */

#if (! defined yyoverflow \
     && (! defined __cplusplus \
         || (defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yy_state_t yyss_alloc;
  YYSTYPE yyvs_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (YYSIZEOF (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (YYSIZEOF (yy_state_t) + YYSIZEOF (YYSTYPE)) \
      + YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYPTRDIFF_T yynewbytes;                                         \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * YYSIZEOF (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / YYSIZEOF (*yyptr);                        \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, YY_CAST (YYSIZE_T, (Count)) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYPTRDIFF_T yyi;                      \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  3
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   3170

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  216
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  115
/* YYNRULES -- Number of rules.  */
#define YYNRULES  417
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  729

/* YYMAXUTOK -- Last valid token kind.  */
#define YYMAXUTOK   446


/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
#define YYTRANSLATE(YYX)                                \
  (0 <= (YYX) && (YYX) <= YYMAXUTOK                     \
   ? YY_CAST (yysymbol_kind_t, yytranslate[YYX])        \
   : YYSYMBOL_YYUNDEF)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static const yytype_uint8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,   200,     2,     2,     2,   199,   192,     2,
     213,   214,   197,   196,   187,   195,   207,   198,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,   189,   212,
     193,   215,   194,   188,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,   208,     2,   209,   191,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,   210,   190,   211,   201,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    58,    59,    60,    61,    62,    63,    64,
      65,    66,    67,    68,    69,    70,    71,    72,    73,    74,
      75,    76,    77,    78,    79,    80,    81,    82,    83,    84,
      85,    86,    87,    88,    89,    90,    91,    92,    93,    94,
      95,    96,    97,    98,    99,   100,   101,   102,   103,   104,
     105,   106,   107,   108,   109,   110,   111,   112,   113,   114,
     115,   116,   117,   118,   119,   120,   121,   122,   123,   124,
     125,   126,   127,   128,   129,   130,   131,   132,   133,   134,
     135,   136,   137,   138,   139,   140,   141,   142,   143,   144,
     145,   146,   147,   148,   149,   150,   151,   152,   153,   154,
     155,   156,   157,   158,   159,   160,   161,   162,   163,   164,
     165,   166,   167,   168,   169,   170,   171,   172,   173,   174,
     175,   176,   177,   178,   179,   180,   181,   182,   183,   184,
     185,   186,   202,   203,   204,   205,   206
};

#if YYDEBUG
/* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_int16 yyrline[] =
{
       0,   326,   326,   343,   343,   345,   346,   346,   348,   349,
     350,   353,   356,   357,   358,   361,   362,   363,   363,   365,
     366,   367,   370,   371,   372,   373,   376,   377,   380,   381,
     385,   386,   387,   388,   389,   390,   391,   394,   405,   406,
     410,   411,   412,   413,   414,   415,   416,   417,   418,   421,
     423,   431,   437,   445,   446,   448,   456,   467,   468,   471,
     472,   475,   476,   480,   485,   492,   496,   497,   500,   501,
     505,   508,   509,   510,   513,   514,   517,   518,   519,   520,
     521,   522,   523,   524,   525,   526,   527,   528,   529,   530,
     531,   532,   533,   534,   535,   536,   537,   538,   539,   540,
     541,   542,   543,   544,   545,   546,   547,   548,   549,   550,
     551,   552,   553,   554,   555,   556,   557,   558,   559,   560,
     561,   562,   563,   564,   565,   566,   567,   568,   569,   570,
     571,   572,   573,   574,   575,   576,   577,   578,   579,   580,
     581,   582,   583,   584,   585,   586,   587,   588,   589,   590,
     591,   592,   596,   597,   598,   599,   600,   601,   602,   603,
     604,   605,   606,   607,   608,   609,   610,   611,   612,   613,
     614,   615,   616,   617,   618,   619,   623,   624,   629,   630,
     631,   632,   635,   636,   639,   643,   649,   650,   651,   654,
     658,   670,   674,   679,   682,   683,   686,   687,   690,   691,
     692,   693,   694,   695,   696,   697,   698,   699,   700,   701,
     702,   703,   704,   705,   706,   707,   708,   709,   710,   711,
     712,   713,   714,   715,   716,   717,   718,   719,   720,   721,
     722,   723,   724,   725,   726,   727,   729,   731,   732,   735,
     736,   739,   745,   751,   752,   755,   760,   767,   768,   771,
     772,   776,   777,   780,   784,   790,   798,   802,   807,   808,
     811,   812,   813,   816,   818,   821,   822,   823,   824,   825,
     826,   827,   828,   829,   830,   831,   834,   835,   838,   839,
     840,   841,   842,   843,   844,   845,   846,   849,   850,   858,
     864,   868,   871,   872,   876,   879,   880,   883,   892,   893,
     896,   897,   900,   906,   912,   913,   916,   917,   920,   930,
     940,   946,   950,   951,   954,   955,   958,   963,   970,   971,
     972,   976,   980,   983,   984,   987,   988,   992,   993,   997,
     998,   999,  1003,  1005,  1007,  1011,  1012,  1013,  1014,  1022,
    1024,  1026,  1031,  1033,  1038,  1039,  1044,  1045,  1046,  1047,
    1052,  1061,  1063,  1064,  1069,  1071,  1075,  1076,  1083,  1084,
    1085,  1086,  1087,  1092,  1100,  1101,  1104,  1105,  1108,  1115,
    1116,  1121,  1122,  1126,  1127,  1128,  1129,  1130,  1134,  1135,
    1136,  1139,  1142,  1143,  1144,  1145,  1146,  1147,  1148,  1149,
    1150,  1151,  1154,  1161,  1163,  1169,  1170,  1171,  1175,  1176,
    1180,  1181,  1185,  1192,  1201,  1202,  1206,  1207,  1211,  1213,
    1214,  1215,  1219,  1220,  1225,  1226,  1227,  1228
};
#endif

/** Accessing symbol of state STATE.  */
#define YY_ACCESSING_SYMBOL(State) YY_CAST (yysymbol_kind_t, yystos[State])

#if 1
/* The user-facing name of the symbol whose (internal) number is
   YYSYMBOL.  No bounds checking.  */
static const char *yysymbol_name (yysymbol_kind_t yysymbol) YY_ATTRIBUTE_UNUSED;

/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "\"end of file\"", "error", "\"invalid token\"", "aIDENTIFIER",
  "aPRAGMA", "aKNOWNTYPE", "aNUM", "aHEXNUM", "aDOUBLE", "aSTRING",
  "aWSTRING", "aSQSTRING", "aUUID", "aEOF", "aACF", "SHL", "SHR",
  "MEMBERPTR", "EQUALITY", "INEQUALITY", "GREATEREQUAL", "LESSEQUAL",
  "LOGICALOR", "LOGICALAND", "ELLIPSIS", "tAGGREGATABLE", "tALLNODES",
  "tALLOCATE", "tANNOTATION", "tAPPOBJECT", "tASYNC", "tASYNCUUID",
  "tAUTOHANDLE", "tBINDABLE", "tBOOLEAN", "tBROADCAST", "tBYTE",
  "tBYTECOUNT", "tCALLAS", "tCALLBACK", "tCASE", "tCDECL", "tCHAR",
  "tCOCLASS", "tCODE", "tCOMMSTATUS", "tCONST", "tCONTEXTHANDLE",
  "tCONTEXTHANDLENOSERIALIZE", "tCONTEXTHANDLESERIALIZE", "tCONTROL",
  "tCPPQUOTE", "tDECODE", "tDEFAULT", "tDEFAULTBIND", "tDEFAULTCOLLELEM",
  "tDEFAULTVALUE", "tDEFAULTVTABLE", "tDISABLECONSISTENCYCHECK",
  "tDISPLAYBIND", "tDISPINTERFACE", "tDLLNAME", "tDONTFREE", "tDOUBLE",
  "tDUAL", "tENABLEALLOCATE", "tENCODE", "tENDPOINT", "tENTRY", "tENUM",
  "tERRORSTATUST", "tEXPLICITHANDLE", "tEXTERN", "tFALSE", "tFASTCALL",
  "tFAULTSTATUS", "tFLOAT", "tFORCEALLOCATE", "tHANDLE", "tHANDLET",
  "tHELPCONTEXT", "tHELPFILE", "tHELPSTRING", "tHELPSTRINGCONTEXT",
  "tHELPSTRINGDLL", "tHIDDEN", "tHYPER", "tID", "tIDEMPOTENT", "tIGNORE",
  "tIIDIS", "tIMMEDIATEBIND", "tIMPLICITHANDLE", "tIMPORT", "tIMPORTLIB",
  "tIN", "tIN_LINE", "tINLINE", "tINPUTSYNC", "tINT", "tINT32", "tINT3264",
  "tINT64", "tINTERFACE", "tLCID", "tLENGTHIS", "tLIBRARY", "tLICENSED",
  "tLOCAL", "tLONG", "tMAYBE", "tMESSAGE", "tMETHODS", "tMODULE",
  "tNAMESPACE", "tNOCODE", "tNONBROWSABLE", "tNONCREATABLE",
  "tNONEXTENSIBLE", "tNOTIFY", "tNOTIFYFLAG", "tNULL", "tOBJECT", "tODL",
  "tOLEAUTOMATION", "tOPTIMIZE", "tOPTIONAL", "tOUT", "tPARTIALIGNORE",
  "tPASCAL", "tPOINTERDEFAULT", "tPRAGMA_WARNING", "tPROGID",
  "tPROPERTIES", "tPROPGET", "tPROPPUT", "tPROPPUTREF", "tPROXY", "tPTR",
  "tPUBLIC", "tRANGE", "tREADONLY", "tREF", "tREGISTER", "tREPRESENTAS",
  "tREQUESTEDIT", "tRESTRICTED", "tRETVAL", "tSAFEARRAY", "tSHORT",
  "tSIGNED", "tSINGLENODE", "tSIZEIS", "tSIZEOF", "tSMALL", "tSOURCE",
  "tSTATIC", "tSTDCALL", "tSTRICTCONTEXTHANDLE", "tSTRING", "tSTRUCT",
  "tSWITCH", "tSWITCHIS", "tSWITCHTYPE", "tTHREADING", "tTRANSMITAS",
  "tTRUE", "tTYPEDEF", "tUIDEFAULT", "tUNION", "tUNIQUE", "tUNSIGNED",
  "tUSESGETLASTERROR", "tUSERMARSHAL", "tUUID", "tV1ENUM", "tVARARG",
  "tVERSION", "tVIPROGID", "tVOID", "tWCHAR", "tWIREMARSHAL", "tAPARTMENT",
  "tNEUTRAL", "tSINGLE", "tFREE", "tBOTH", "','", "'?'", "':'", "'|'",
  "'^'", "'&'", "'<'", "'>'", "'-'", "'+'", "'*'", "'/'", "'%'", "'!'",
  "'~'", "CAST", "PPTR", "POS", "NEG", "ADDRESSOF", "'.'", "'['", "']'",
  "'{'", "'}'", "';'", "'('", "')'", "'='", "$accept", "input", "m_acf",
  "gbl_statements", "$@1", "imp_statements", "$@2", "int_statements",
  "semicolon_opt", "statement", "pragma_warning", "warnings", "typedecl",
  "cppquote", "import_start", "import", "importlib", "libraryhdr",
  "library_start", "librarydef", "m_args", "arg_list", "args", "arg",
  "array", "m_attributes", "attributes", "attrib_list", "str_list",
  "attribute", "uuid_string", "callconv", "cases", "case", "enums",
  "enum_list", "enum", "enumdef", "m_exprs", "m_expr", "expr",
  "expr_list_int_const", "expr_int_const", "expr_const", "fields", "field",
  "ne_union_field", "ne_union_fields", "union_field", "s_field", "funcdef",
  "declaration", "m_ident", "t_ident", "ident", "base_type", "m_int",
  "int_std", "coclass", "coclasshdr", "coclassdef", "namespacedef",
  "coclass_ints", "coclass_int", "dispinterface", "dispinterfacehdr",
  "dispint_props", "dispint_meths", "dispinterfacedef", "inherit",
  "interface", "interfacehdr", "interfacedef", "interfacedec", "module",
  "modulehdr", "moduledef", "storage_cls_spec", "function_specifier",
  "type_qualifier", "m_type_qual_list", "decl_spec", "m_decl_spec_no_type",
  "decl_spec_no_type", "declarator", "direct_declarator",
  "abstract_declarator", "abstract_declarator_no_direct",
  "m_abstract_declarator", "abstract_direct_declarator", "any_declarator",
  "any_declarator_no_direct", "m_any_declarator", "any_direct_declarator",
  "declarator_list", "m_bitfield", "struct_declarator",
  "struct_declarator_list", "init_declarator", "threading_type",
  "pointer_type", "structdef", "type", "typedef", "uniondef", "version",
  "acf_statements", "acf_int_statements", "acf_int_statement",
  "acf_interface", "acf_attributes", "acf_attribute_list", "acf_attribute",
  "allocate_option_list", "allocate_option", YY_NULLPTR
};

static const char *
yysymbol_name (yysymbol_kind_t yysymbol)
{
  return yytname[yysymbol];
}
#endif

#define YYPACT_NINF (-564)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-405)

#define yytable_value_is_error(Yyn) \
  0

/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
   STATE-NUM.  */
static const yytype_int16 yypact[] =
{
    -564,   108,  1696,  -564,  -564,  -564,   -66,  -564,  -564,  -564,
     176,  -564,  -101,   195,  -564,   243,  -564,  -564,  -564,  -564,
      38,   148,  -564,  -564,  -564,  -564,  -564,   269,    38,   165,
     -35,  -564,   -14,    38,    15,  -564,  -564,   291,   321,    15,
    -564,  -564,  2989,  -564,  -564,  -564,    47,  -564,  -564,  -564,
    -564,  -564,    51,  2671,    53,    57,  -564,  -564,    64,    24,
    -564,    81,    80,    85,    87,    91,   117,  -564,  -564,   123,
    -564,   -16,   -16,   -16,    48,  2836,   125,   -16,   130,   136,
      96,  -564,   -66,   152,  -564,  -564,   346,  -564,  -564,   119,
    -564,   139,  -564,  -564,   149,  -564,  -564,  -564,  -564,   368,
    2836,  -564,  -564,   122,   168,   -92,  -117,  -564,  -564,   169,
    -564,  -564,   173,  -564,  -564,  -564,   174,   186,  -564,  -564,
    -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,   187,  -564,
    -564,  -564,   188,  -564,  -564,  -564,   189,   190,  -564,  -564,
    -564,  -564,   191,   192,   194,   203,   206,  -564,   207,  -564,
    -564,   208,  -564,   213,  -564,  -564,   214,   216,  -564,  -564,
    -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,
    -564,   217,  -564,  -564,  -564,   218,   220,  -564,  -564,  -564,
    -564,  -564,  -564,   223,  -564,  -564,   224,  -564,  -564,  -564,
     225,  -564,  -564,  -564,   226,   228,   230,   231,  -564,  -564,
    -564,   236,   257,  -564,  -564,   259,   260,   262,   -62,  -564,
    -564,  -564,  1836,   878,   200,   335,   340,   349,   359,   364,
     267,   268,  -564,  -564,  -564,  -564,    48,   270,   271,  -564,
    -564,  -564,  -564,  -564,   -30,  -564,  -564,  -564,   369,   274,
    -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,
    -564,  -564,    48,    48,  -564,   272,   -58,  -564,  -564,  -564,
     -16,  -564,  -564,  -564,   275,  -564,  -564,  -564,   -12,  -564,
    -564,   423,   278,   376,  -564,   296,   279,  -564,   276,  -564,
     485,    43,   376,   731,   731,   487,   489,   731,   731,   490,
     491,   731,   492,   731,   731,  2224,   731,   731,   494,   -80,
     495,   731,  2836,   731,   731,  2836,    98,  2836,  2836,    43,
     202,   496,  2836,  2989,   298,  -564,   295,  -564,  -564,  -564,
     299,  -564,   302,  -564,  -564,  -564,    87,  2760,  -564,   303,
    -564,  -564,  -564,  -564,   303,   -97,  -564,  -564,  -112,  -564,
     325,   -72,   305,   308,  -564,  -564,  1239,    45,   306,  -564,
     731,   624,  2224,  -564,  -564,    66,    96,  -564,   311,  -564,
     312,   337,  -564,   307,   520,  -564,  -111,   200,    59,   313,
    -564,  -564,   314,   316,  -564,  -564,  -564,  -564,  -564,  -564,
    -564,  -564,  -564,   318,  -564,   731,   731,   731,   731,   731,
     731,   605,  2473,   -93,  -564,  2473,   320,   322,  -564,   -42,
     328,   330,   331,   332,   333,   334,   336,   374,   338,  2760,
      65,   339,   -41,  -564,  2473,   341,   342,   344,   345,   347,
     -38,  2230,   360,  -564,  -564,  -564,  -564,  -564,   361,   362,
     363,   365,   352,  -564,   366,   370,   372,  -564,  2989,   526,
    -564,  -564,  -564,    48,    87,    27,  -564,  1120,  -564,   371,
    2760,   351,  1575,   343,   456,  1358,    87,  -564,  2760,  -564,
    -564,  -564,  -564,   709,  -564,  2390,   373,   391,  -564,  -564,
    -564,  -564,  -564,  -564,   -34,  -564,  -564,   384,  -564,   376,
     731,  -564,    21,  -564,  2760,  -564,   377,  -564,   378,  -564,
     380,  -564,  -564,  -564,  2760,    32,    32,    32,    32,    32,
      32,  2319,   227,   731,   731,   590,   731,   731,   731,   731,
     731,   731,   731,   731,   731,   731,   731,   731,   731,   731,
     731,   731,   731,   592,   731,   731,  -564,  -564,  -564,   587,
    -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,  -564,
      65,  -564,  1957,  -564,    65,  -564,  -564,  -564,     8,  -564,
     731,  -564,  -564,  -564,  -564,   731,  -564,  -564,  -564,  -564,
    -564,  -564,  -564,  -564,   591,  -564,  -564,  -564,  -564,   385,
    -564,  -564,   411,  -564,  -564,  -564,  -564,    48,   171,  -564,
    -564,  2760,   388,  -564,  -564,  -564,    87,  -564,  -564,  -564,
    -564,  2135,    66,  -564,   393,   394,   384,  -564,  -564,  -564,
    -564,    65,   390,   376,  -564,  -564,   227,  -564,  -564,  2046,
    -564,   227,  -564,   392,    28,    90,    90,  -564,   672,   672,
     215,   215,  2345,  2504,  2452,  1277,  1396,  1337,   215,   215,
      30,    30,    32,    32,    32,  -564,  2430,  -564,  -564,  -564,
      36,  -564,   395,    65,   403,  -564,  2224,  -564,  -564,   404,
    -564,    87,   999,    48,  -564,  -564,  1477,  -564,  -564,  -564,
    -564,   599,  -564,  -564,   430,  -564,  -103,  -564,   410,  -564,
     407,   289,  -564,   408,    65,   409,  -564,   731,  2224,  -564,
     731,  -564,  -564,    36,  -564,  -564,  -564,   414,  -564,  -564,
    -564,  -564,    87,   412,   731,  -564,    65,  -564,  -564,  -564,
    -564,    36,  -564,  -564,  -564,    32,   415,  2473,  -564,  -564,
    -564,  -564,  -564,  -564,    -7,  -564,  -564,   731,   436,  -564,
    -564,   447,  -147,  -147,  -564,  -564,   425,  -564,  -564
};

/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
   Performed when YYTABLE does not specify something else to do.  Zero
   means the default is an error.  */
static const yytype_int16 yydefact[] =
{
       5,     0,     3,     1,    35,   383,   398,   273,   265,   284,
       0,   322,     0,     0,   272,   260,   274,   318,   271,   275,
     276,     0,   321,   278,   285,   286,   283,     0,   276,     0,
       0,   320,     0,   276,     0,   280,   319,   260,   260,   270,
     382,   266,    76,     2,    14,    36,     0,    30,    15,    33,
      15,    13,     0,    69,   385,     0,   384,   267,     0,     0,
      11,     0,     0,     0,    28,     0,   304,     9,     8,     0,
      12,   327,   327,   327,     0,     0,   387,   327,     0,   389,
       0,     4,   398,     0,   287,   288,     0,   295,   296,   386,
     262,     0,   277,   282,     0,   306,   307,   281,   291,     0,
       0,   279,   268,   388,     0,   390,     0,   269,    77,     0,
      79,    80,     0,    81,    82,    83,     0,     0,    86,    87,
      88,    89,    90,    91,    92,    93,    94,    95,     0,    97,
      98,    99,     0,   101,   102,   103,     0,     0,   106,   107,
     108,   109,     0,     0,     0,     0,     0,   115,     0,   117,
     118,     0,   120,     0,   122,   123,   126,     0,   127,   128,
     129,   130,   131,   132,   133,   134,   135,   136,   137,   138,
     139,     0,   141,   142,   143,     0,     0,   146,   147,   148,
     149,   380,   150,     0,   152,   378,     0,   154,   155,   156,
       0,   158,   159,   160,     0,     0,     0,     0,   165,   379,
     166,     0,     0,   170,   171,     0,     0,     0,     0,    71,
     175,    31,    68,    68,    68,   260,     0,     0,   260,   260,
       0,   385,   289,   297,   308,   316,     0,   387,   389,    32,
      10,   292,     6,   313,     0,    29,   311,   312,     0,     0,
      26,   331,   328,   330,   329,   263,   264,   178,   179,   180,
     181,   323,     0,     0,   335,   371,   334,   257,   385,   387,
     327,   389,   325,    34,     0,   410,   409,   411,     0,   406,
     399,     0,     0,   186,    50,     0,     0,   243,     0,   249,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   196,     0,     0,
       0,     0,     0,   196,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    76,    70,    51,     0,    23,    24,    25,
       0,    21,     0,    19,    16,    22,    28,     0,    69,   386,
      53,    54,   314,   315,   388,   390,    55,   256,    68,     5,
       0,    68,     0,     0,   305,    26,    68,     0,     0,   333,
       0,     0,    57,   337,   326,     0,     0,   405,     0,    49,
       0,   188,   189,   192,     0,   391,    68,    68,    68,     0,
     177,   176,     0,     0,   207,   198,   199,   200,   204,   205,
     206,   201,   202,     0,   203,     0,     0,     0,     0,     0,
       0,     0,   241,     0,   239,   242,     0,     0,    74,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     356,     0,     0,   194,   197,     0,     0,     0,     0,     0,
       0,     0,     0,   373,   374,   375,   376,   377,     0,     0,
       0,     0,   395,   397,     0,     0,     0,    72,    76,     0,
      20,    17,    56,     0,    28,     0,   293,    68,   298,     0,
       0,     0,     0,     0,     0,    68,    28,    27,    69,   324,
     332,   336,   372,     0,    67,     0,     0,    61,    58,    59,
     416,   414,   417,   415,     0,   412,   407,   400,   193,   187,
       0,    38,     0,   381,     0,   244,     0,   393,    69,   250,
       0,    78,   169,    84,     0,   231,   230,   229,   232,   227,
     228,     0,   344,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    85,    96,   100,     0,
     104,   105,   110,   111,   112,   113,   114,   116,   119,   121,
     356,   323,    57,   361,   356,   358,   357,    64,   353,   125,
     196,   124,   140,   144,   145,     0,   153,   157,   161,   162,
     164,   163,   167,   168,     0,   172,   173,   174,    73,     0,
      15,   364,   392,   290,   294,     7,   300,     0,   387,   299,
     302,     0,     0,   255,   303,    26,    28,   317,    66,    65,
     338,     0,     0,   408,   404,     0,   400,   190,   191,    39,
      37,     0,   389,   258,   248,   247,   344,   238,   323,    57,
     348,   344,   345,     0,   341,   220,   221,   233,   214,   215,
     218,   219,   209,   210,     0,   211,   212,   213,   217,   216,
     223,   222,   225,   226,   224,   234,     0,   240,    75,    63,
     356,   323,     0,   356,     0,   352,    57,   360,   195,     0,
     396,    28,    68,     0,   253,   301,    68,   309,    62,    60,
     413,     0,   403,   401,   366,   369,     0,   246,     0,   259,
       0,   344,   323,     0,   356,     0,   340,     0,    57,   347,
       0,   237,   351,   356,   362,   355,   359,     0,   151,    52,
      18,   365,    28,     0,     0,   368,     0,   245,   182,   236,
     339,   356,   349,   343,   346,   235,     0,   208,   354,   363,
     310,   402,   367,   370,     0,   342,   350,     0,     0,   394,
     183,     0,    68,    68,   252,   185,     0,   184,   251
};

/* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -564,  -564,  -564,   301,  -564,   -45,  -564,  -331,  -315,     0,
    -564,  -564,  -564,  -564,  -564,   184,  -564,  -564,  -564,    11,
    -483,  -564,  -564,  -263,  -246,  -207,    -2,  -564,  -564,  -284,
     353,   -68,  -564,  -564,  -564,  -564,   163,    13,   355,    93,
    -199,  -564,  -265,  -279,  -564,  -564,  -564,  -564,   -78,  -197,
    -564,   196,  -564,     5,   -70,  -564,   110,   162,    10,  -564,
      17,    19,  -564,  -564,   593,  -564,  -564,  -564,  -564,  -564,
     -28,  -564,    20,    16,  -564,  -564,    22,  -564,  -564,  -313,
    -506,   -52,     6,     3,  -236,  -564,  -564,  -564,  -539,  -564,
    -563,  -564,  -164,  -564,  -564,  -564,   -47,  -564,   426,  -564,
     356,     1,   -55,  -564,     7,  -564,   578,    68,  -564,  -564,
      67,  -564,   310,  -564,    75
};

/* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
       0,     1,    43,     2,   339,   212,   570,   346,   236,   317,
      45,   482,    46,    47,    48,    49,   318,   220,    50,   319,
     466,   467,   468,   469,   543,    52,   328,   208,   399,   209,
     372,   544,   714,   720,   360,   361,   362,   258,   412,   413,
     392,   393,   394,   396,   366,   485,   489,   368,   725,   726,
     582,    55,   668,    91,   545,    56,    93,    57,   320,    59,
     321,   322,   338,   446,    62,    63,   341,   452,    64,   239,
      65,    66,   323,   324,   225,    69,   325,    71,    72,    73,
     347,    74,   241,    75,   255,   256,   612,   675,   613,   614,
     546,   644,   547,   548,   572,   695,   665,   666,   257,   428,
     210,   259,    77,    78,   261,   434,    81,   595,   596,    82,
      83,   268,   269,   474,   475
};

/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
   positive, shift that token.  If negative, reduce the rule whose
   number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_int16 yytable[] =
{
      53,   226,    44,    76,   254,   213,   253,   327,   400,    79,
     353,   442,    58,    51,   455,    54,   348,   349,    68,    60,
     260,    61,    67,   401,    70,   224,   404,   599,   406,   437,
      11,   411,   408,   717,   459,   640,   418,  -404,   664,   245,
     449,   246,   104,   106,   278,   276,   718,   505,   245,   505,
     246,   245,   370,   246,   227,   371,    17,     9,   181,   642,
     228,    42,   185,   222,  -261,   724,   221,   670,   245,  -261,
     246,   462,   676,    27,   242,   242,   242,   247,   243,   244,
     242,    22,    11,   262,   696,   395,   247,    13,   395,   247,
     199,    11,   470,   279,   525,   407,    42,    42,   414,   444,
     483,    20,   671,   340,   414,   421,   247,   505,     3,   697,
     248,   460,    86,  -261,    23,    24,    25,    26,  -261,   248,
     -45,   526,   248,   264,    28,   313,   673,    31,   471,   573,
      27,   445,   700,   664,   450,   683,    42,    92,    97,   248,
      36,   587,    80,   101,   451,   529,   550,   314,   265,   550,
     351,   395,   465,   592,   568,   352,   254,    94,   253,   484,
     450,   450,   266,   687,    33,   249,   701,   267,    98,    35,
     486,   490,   530,   551,   249,   356,   557,   249,    99,    84,
     593,    85,   254,   254,   253,   253,   495,   496,   497,   498,
     499,   500,   501,   250,   249,   706,   102,   357,    87,   100,
      88,   107,   250,   363,   719,   250,   342,   571,   432,   433,
      53,    53,   373,    76,    76,   598,   351,   472,   214,    79,
      79,   646,   250,   104,   106,    54,    54,   520,   521,   522,
     503,   504,   505,   541,   231,   600,   351,   523,   524,   523,
     524,   678,   251,   410,   351,   251,    89,   419,    90,   542,
     422,   473,   429,   430,   656,   271,   610,   436,   252,   211,
     637,   252,   541,   242,   498,   -40,   354,    42,   247,   229,
     487,   657,    95,   351,    96,   443,   230,   254,   542,   253,
     423,   424,   425,   426,   427,   518,   519,   520,   521,   522,
     649,   232,   233,   409,   103,   234,    90,   523,   524,   235,
     410,   248,   647,   237,   615,   616,   238,   618,   619,   620,
     621,   622,   623,   624,   625,   626,   627,   628,   629,   630,
     631,   632,   633,   634,   105,   636,    90,   459,   659,  -261,
     247,   -41,  -261,   240,   -43,    11,   689,   -42,   329,   502,
      90,   654,   263,   330,   458,   331,   457,    76,   -44,   273,
     409,   414,   332,    79,   333,   272,   249,   540,   459,    54,
     610,   274,   334,   248,    90,   610,   488,   335,   679,    90,
     459,   275,   343,   254,   344,   253,   639,   710,   277,   245,
     645,   246,   280,  -254,   250,  -254,   281,   282,   459,   503,
     504,   505,   506,   507,   508,   509,   510,   511,   577,   283,
     284,   285,   286,   287,   288,   289,   226,   290,    42,   363,
     518,   519,   520,   521,   522,   712,   291,   691,   249,   292,
     293,   294,   523,   524,   608,   610,   295,   296,   358,   297,
     298,   299,   601,   300,   611,   351,   301,   302,   303,   304,
     609,   305,   606,   306,   307,    53,   250,    44,    76,   308,
     581,   578,   721,   458,    79,   457,    76,    58,    51,   227,
      54,   574,    79,    68,    60,   228,    61,    67,    54,    70,
     309,   221,   310,   311,   643,   312,   682,   336,   705,   685,
     -46,   707,   -47,   -48,   345,   364,   608,   350,   355,   367,
     410,   602,   359,   365,   369,   395,   397,   351,   398,   402,
     403,   405,   609,   415,   417,   435,   438,   254,   439,   253,
     703,   440,   441,  -261,   448,   450,   450,   453,   454,   708,
     461,   477,   480,   478,   479,   652,   481,   491,   492,   226,
     493,   494,   555,   669,   527,   569,   528,   715,   611,   410,
     409,   674,   531,   611,   532,   533,   534,   535,   536,    21,
     537,   594,   539,   549,   584,   552,   553,   410,   554,   564,
     576,   556,   512,   579,   513,   514,   515,   516,   517,   518,
     519,   520,   521,   522,   559,   560,   561,   562,   591,   563,
     565,   523,   524,   254,   566,   253,   567,   590,   538,   409,
     604,   603,   605,   617,   410,   635,   638,   650,   653,   651,
     655,    80,   667,   611,   693,   662,   677,   409,   374,   684,
       5,   375,   376,   377,   378,   379,   380,   686,   688,   694,
     698,   699,   702,   704,   711,   722,   410,   374,   709,   716,
     375,   376,   377,   378,   379,   380,   723,   728,   585,     7,
     447,     8,   597,   648,   409,   727,   223,     9,   583,   713,
      53,    11,   337,    76,   458,   416,   457,    76,   420,    79,
     270,   661,   431,    79,   663,    54,   476,   660,    14,    54,
       0,     0,     0,     0,   215,    16,   409,    17,   381,     0,
       0,    18,     0,     0,    19,     0,     0,   503,   504,   505,
       0,    20,   508,   509,     0,     0,     0,   381,     0,     0,
       0,     0,    22,     0,    23,    24,    25,    26,     0,     0,
       0,     0,   374,     0,    28,   375,   376,   377,   378,   379,
     380,     0,     0,     0,     0,     0,   382,     0,     0,     0,
       0,     0,     0,     0,   374,     0,     0,   375,   376,   377,
     378,   379,   380,     0,     0,   382,     0,     0,    31,     0,
       0,     0,     0,    32,    33,    34,     0,     0,   383,    35,
       0,    36,     0,     0,     0,   218,     0,     0,     0,     0,
       0,   384,     0,     0,   219,     0,    39,   383,     0,     0,
       0,     0,   381,     0,    40,    41,     0,     0,     0,     0,
     384,     0,     0,     0,     0,     0,     0,   385,     0,     0,
     386,   387,   388,     0,   381,   389,   390,     0,     0,     0,
       0,     0,     0,     0,     0,     0,   385,     0,   391,   386,
     387,   463,     0,     0,   389,   390,     0,     0,     0,     0,
     382,     0,     0,   464,     0,     0,     0,   391,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   382,     0,     0,     0,     0,     0,     0,     0,
       0,     0,   383,     0,     0,   516,   517,   518,   519,   520,
     521,   522,     0,     0,     0,   384,     0,     0,     0,   523,
     524,     0,     4,     5,   383,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   384,     0,     0,
       0,   385,     0,     0,   386,   387,   388,     0,     0,   389,
     390,     0,     7,     0,     8,     0,     0,     0,   588,     0,
       9,    10,   391,   385,    11,     0,   386,   387,   388,    12,
       0,   389,   390,     0,     0,     0,     0,     0,    13,     0,
       0,    14,     0,     0,   391,     0,     0,    15,    16,     0,
      17,     0,     0,     0,    18,     0,     0,    19,     0,     0,
       0,     0,     0,     0,    20,     0,     0,     0,     0,     0,
       0,    21,   316,     0,     0,    22,     0,    23,    24,    25,
      26,    27,     0,     0,     0,     0,     0,    28,     0,     0,
       0,     0,    29,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     4,     5,     0,     0,     0,     0,    30,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    31,     0,     0,     0,     0,    32,    33,    34,     0,
       0,     0,    35,     7,    36,     8,     0,     0,    37,     0,
       0,     9,    10,     0,     0,    11,     0,    38,     0,    39,
      12,     0,     0,     0,     0,     0,     0,    40,    41,    13,
       0,     0,    14,     0,     0,     0,     0,     0,    15,    16,
       0,    17,     0,     0,     0,    18,     0,     0,    19,     0,
       0,     0,     0,     0,     0,    20,    42,     0,     0,   326,
       0,     0,    21,   316,     0,     0,    22,     0,    23,    24,
      25,    26,    27,     0,     0,     0,     0,     0,    28,     0,
       0,     0,     0,    29,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     4,     5,     0,     0,     0,     0,
      30,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    31,     0,     0,     0,     0,    32,    33,    34,
       0,     0,     0,    35,     7,    36,     8,     0,     0,    37,
       0,     0,     9,    10,     0,     0,    11,     0,    38,     0,
      39,    12,     0,     0,     0,     0,     0,     0,    40,    41,
      13,     0,     0,    14,     0,     0,     0,     0,     0,    15,
      16,     0,    17,     0,     0,     0,    18,     0,     0,    19,
       0,     0,     0,     0,     0,     0,    20,    42,     0,     0,
     690,     0,     0,    21,     0,     0,     0,    22,     0,    23,
      24,    25,    26,    27,     0,     0,     0,     0,     0,    28,
       0,     0,     0,     0,    29,     0,     0,     0,     0,     0,
       0,     0,     0,     4,     5,     0,     0,     0,     0,     0,
       0,    30,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,    31,     0,     0,     0,     0,    32,    33,
      34,     0,     0,     7,    35,     8,    36,     0,     0,     0,
      37,     9,     0,     0,     0,    11,     0,     0,     0,    38,
      12,    39,   503,   504,   505,   506,   507,   508,   509,    40,
      41,     0,    14,     0,     0,     0,     0,     0,    15,    16,
       0,    17,     0,     0,     0,    18,     0,     0,    19,     0,
       0,     0,     0,     0,     0,    20,     0,     0,    42,     0,
       0,   575,    21,     0,     0,     0,    22,     0,    23,    24,
      25,    26,     0,     0,     0,     0,     0,     0,    28,     0,
       0,     0,   503,   504,   505,   506,   507,   508,   509,     0,
       0,     0,     4,     5,     0,     0,     0,     0,     0,     0,
      30,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    31,     0,     0,     0,     0,    32,    33,    34,
       0,     0,     7,    35,     8,    36,     0,     0,     0,    37,
       9,     0,     0,     0,    11,     0,     0,     0,    38,    12,
      39,   503,   504,   505,   506,   507,   508,   509,    40,    41,
       0,    14,     0,     0,     0,     0,     0,    15,    16,     0,
      17,     0,     0,     0,    18,     0,     0,    19,     0,     0,
       0,     0,     0,     0,    20,     0,     0,    42,     0,     0,
     456,    21,     0,     0,     0,    22,     0,    23,    24,    25,
      26,     0,     0,     0,     0,     0,     0,    28,   514,   515,
     516,   517,   518,   519,   520,   521,   522,     0,     0,     0,
       0,     4,     5,     0,   523,   524,     0,     0,     0,    30,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    31,     0,     0,     0,     0,    32,    33,    34,     0,
       0,     7,    35,     8,    36,     0,     0,     0,    37,     9,
       0,     0,     0,    11,     0,     0,     0,    38,    12,    39,
     516,   517,   518,   519,   520,   521,   522,    40,    41,     0,
      14,     0,     0,     0,   523,   524,    15,    16,     0,    17,
       0,     0,     0,    18,     0,     0,    19,     0,     0,     0,
       0,     0,     0,    20,     0,     0,    42,     0,     0,   586,
      21,     0,     0,     0,    22,     0,    23,    24,    25,    26,
       5,     0,     0,     0,     0,     0,    28,     0,   515,   516,
     517,   518,   519,   520,   521,   522,     0,     0,     0,     0,
       0,     0,     0,   523,   524,     0,     0,     0,    30,     7,
       0,     8,     0,     0,     0,     0,     0,     9,     0,     0,
      31,    11,     0,     0,     0,    32,    33,    34,     0,     0,
       0,    35,     0,    36,     0,     0,     0,    37,    14,     0,
       0,     0,     0,     0,   215,    16,    38,    17,    39,     0,
       0,    18,     0,     0,    19,     0,    40,    41,     0,     0,
       0,    20,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,    22,     0,    23,    24,    25,    26,     0,     0,
       0,     0,     0,     0,    28,    42,     0,     0,   692,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       4,     5,     0,     0,     0,     0,     0,     0,     0,     0,
       6,     0,     0,     0,     0,     0,     0,     0,    31,     0,
       0,     0,     0,    32,    33,    34,     0,     0,     0,    35,
       7,    36,     8,     0,     0,   218,     0,     0,     9,    10,
       0,     0,    11,     0,   219,     0,    39,    12,     0,     0,
       0,     0,     0,     0,    40,    41,    13,     0,     0,    14,
       0,     0,     0,     0,     0,    15,    16,     0,    17,     0,
       0,     0,    18,     0,     0,    19,     0,     0,     0,     0,
       0,     0,    20,    42,     0,     0,   580,     0,     0,    21,
       0,     0,     0,    22,     0,    23,    24,    25,    26,    27,
       0,     0,     0,     0,     0,    28,     0,     0,     0,     0,
      29,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,    30,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    31,
       4,     5,     0,     0,    32,    33,    34,     0,     0,   315,
      35,     0,    36,     0,     0,     0,    37,     0,     0,     0,
       0,     0,     0,   -68,     0,    38,     0,    39,     0,     0,
       7,     0,     8,     0,     0,    40,    41,     0,     9,    10,
       0,     0,    11,     0,     0,     0,     0,    12,     0,     0,
       0,     0,     0,     0,     0,     0,    13,     0,     0,    14,
       0,     0,     0,     0,    42,    15,    16,     0,    17,     0,
       0,     0,    18,     0,     0,    19,     0,     0,     0,     0,
       0,     0,    20,     0,     0,     0,     0,     0,     0,    21,
     316,     0,     0,    22,     0,    23,    24,    25,    26,    27,
       0,     0,     0,     0,     0,    28,     0,     0,     0,     0,
      29,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     5,     0,     0,     0,     0,    30,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    31,
       0,     0,     0,     0,    32,    33,    34,     0,     0,     0,
      35,     7,    36,     8,     0,     0,    37,     0,   247,     9,
       0,     0,     0,    11,     0,    38,     0,    39,     0,     0,
       0,     0,     0,     0,     0,    40,    41,     0,     0,     0,
      14,     0,     0,     0,     0,     0,   215,    16,     0,    17,
       0,   248,     0,    18,     0,     0,    19,     0,     0,     0,
       0,     0,     0,    20,    42,     0,     0,     0,     0,     0,
       0,     5,     0,     0,    22,     0,    23,    24,    25,    26,
       0,     0,     0,     0,     0,     0,    28,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       7,     0,     8,     0,     0,     0,   249,   247,     9,     0,
       0,     0,    11,     0,     0,     0,     0,     0,     0,     0,
      31,     0,     0,     0,     0,    32,    33,    34,     0,    14,
       0,    35,     0,    36,   250,   215,    16,   218,    17,     0,
     248,     0,    18,     0,     0,    19,   219,     0,    39,     0,
       0,     0,    20,     0,     0,     0,    40,    41,     0,     0,
       5,     0,     0,    22,     0,    23,    24,    25,    26,     0,
       0,     0,     0,     0,   641,    28,     0,     0,     0,   658,
       0,     0,     0,     0,     0,    42,     0,     0,     0,     7,
       0,     8,     0,     0,     0,   249,     0,     9,     0,     0,
       0,    11,     0,     0,     0,     0,     0,     0,     0,    31,
       0,     0,     0,     0,    32,    33,    34,     0,    14,     0,
      35,     0,    36,   250,   215,    16,   218,    17,     0,     0,
       0,    18,     0,     0,    19,   219,     0,    39,     0,     0,
       0,    20,     0,     0,     0,    40,    41,     0,     0,     5,
       0,     0,    22,     0,    23,    24,    25,    26,     0,     0,
       0,     0,     0,   672,    28,   503,   504,   505,   506,   507,
     508,   509,   510,   511,    42,     0,     0,     0,     7,     0,
       8,     0,     0,     0,     0,     0,     9,     0,     0,     0,
      11,     0,     0,     0,     0,     0,     0,     0,    31,     0,
       0,     0,     0,    32,    33,    34,     0,    14,     0,    35,
       0,    36,     0,   215,    16,   218,    17,     0,     0,     0,
      18,     0,     0,    19,   219,     0,    39,     0,     0,     0,
      20,     0,     0,     0,    40,    41,     0,     0,     0,     0,
       0,    22,     0,    23,    24,    25,    26,     0,     0,     0,
       0,     0,     0,    28,   503,   504,   505,   506,   507,   508,
     509,   510,   511,    42,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     503,   504,   505,   506,   507,   508,   509,    31,   511,     0,
       0,     0,    32,    33,    34,     0,     0,     0,    35,     0,
      36,     0,     0,     0,   218,     0,     0,     0,     0,     0,
       0,     0,     0,   219,     0,    39,     0,     0,     0,     0,
       0,     0,     0,    40,    41,   503,   504,   505,   506,   507,
     508,   509,   510,   511,     0,     0,     0,     0,   512,     0,
     513,   514,   515,   516,   517,   518,   519,   520,   521,   522,
       0,     0,    42,     0,     0,     0,     0,   523,   524,     0,
       0,     0,     0,     0,   558,   503,   504,   505,   506,   507,
     508,   509,   510,   511,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   503,   504,   505,
     506,   507,   508,   509,   510,   511,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   503,   504,
     505,   506,   507,   508,   509,   510,   511,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   512,     0,   513,
     514,   515,   516,   517,   518,   519,   520,   521,   522,   503,
     504,   505,   506,   507,   508,   509,   523,   524,     0,     0,
       0,     0,     0,   607,     0,   513,   514,   515,   516,   517,
     518,   519,   520,   521,   522,     0,     0,     0,     0,     0,
       0,     0,   523,   524,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   512,     0,
     513,   514,   515,   516,   517,   518,   519,   520,   521,   522,
       0,     0,     0,     0,     0,     0,     0,   523,   524,   589,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   512,     0,
     513,   514,   515,   516,   517,   518,   519,   520,   521,   522,
       0,     0,     0,     0,     0,     0,     0,   523,   524,   681,
     512,   680,   513,   514,   515,   516,   517,   518,   519,   520,
     521,   522,     0,     0,     0,     0,     0,     0,     0,   523,
     524,   512,     0,   513,   514,   515,   516,   517,   518,   519,
     520,   521,   522,     0,     0,     0,     5,     0,     0,     0,
     523,   524,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,   513,   514,   515,   516,   517,   518,
     519,   520,   521,   522,     0,     7,     0,     8,     0,     0,
       0,   523,   524,     9,    10,     0,     0,    11,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    13,     0,     0,    14,     0,     0,     0,     0,     0,
     215,    16,     0,    17,     0,     0,     0,    18,     0,     0,
      19,     0,     0,     0,     0,     0,     0,    20,     0,     0,
       0,     0,     0,     0,     0,     5,     0,     0,    22,     0,
      23,    24,    25,    26,    27,     0,     0,   216,     0,     0,
      28,     0,     0,     0,   217,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     7,     0,     8,     0,     0,     0,
       0,     0,     9,     0,     0,     0,    11,     0,     0,     0,
       0,     0,     0,     0,    31,     0,     0,     0,     0,    32,
      33,    34,     0,    14,     0,    35,     0,    36,     0,   215,
      16,   218,    17,     0,     0,     0,    18,     0,     0,    19,
     219,     5,    39,     0,     0,     0,    20,     0,     0,     0,
      40,    41,     0,     0,     0,     0,     0,    22,     0,    23,
      24,    25,    26,     0,     0,     0,     0,     0,     0,    28,
       7,     0,     8,     0,     0,     0,     0,     0,     9,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    14,
       0,     0,     0,    31,     0,   215,    16,     0,    32,    33,
      34,     0,    18,     0,    35,    19,    36,     0,     0,     0,
     218,     0,    20,     0,     0,     0,     0,     0,     0,   219,
       0,    39,     0,     0,     0,    23,    24,    25,    26,    40,
      41,     0,     0,     0,     0,    28,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,    32,    33,    34,     0,     0,     0,
      35,     0,     0,     0,     0,     0,   218,     0,     0,     0,
       0,     0,     0,     0,     0,   219,     0,    39,     0,     0,
       0,     0,     0,     0,   108,    40,    41,   109,   110,   111,
     112,   113,   114,     0,   115,     0,     0,   116,     0,   117,
       0,     0,     0,   118,   119,     0,   120,   121,   122,   123,
       0,   124,   125,   126,   127,   128,   129,   130,   131,     0,
     132,     0,     0,   133,   134,   135,   136,   137,     0,     0,
     138,     0,     0,     0,   139,     0,   140,   141,     0,   142,
     143,   144,   145,   146,   147,     0,   148,   149,   150,   151,
     152,   153,     0,     0,   154,     0,     0,   155,     0,     0,
       0,     0,     0,   156,   157,     0,   158,   159,     0,   160,
     161,     0,     0,     0,   162,   163,   164,   165,   166,   167,
       0,   168,   169,   170,   171,   172,   173,   174,     0,   175,
       0,   176,     0,   177,   178,   179,   180,   181,   182,   183,
     184,   185,     0,   186,   187,   188,   189,     0,     0,     0,
       0,   190,     0,     0,   191,     0,     0,   192,   193,     0,
       0,   194,   195,   196,   197,     0,     0,   198,     0,   199,
       0,   200,   201,   202,   203,   204,   205,   206,     0,     0,
     207
};

static const yytype_int16 yycheck[] =
{
       2,    53,     2,     2,    74,    50,    74,   214,   287,     2,
     256,   326,     2,     2,   345,     2,   252,   253,     2,     2,
      75,     2,     2,   288,     2,    53,   291,     6,   293,   313,
      46,   296,   295,    40,   347,   541,   301,   103,   601,     3,
     112,     5,    37,    38,   161,   100,    53,    17,     3,    17,
       5,     3,     9,     5,    53,    12,    72,    42,   138,   542,
      53,   208,   142,    53,   161,   212,    53,   606,     3,   161,
       5,   350,   611,   103,    71,    72,    73,    41,    72,    73,
      77,    97,    46,    77,   187,   284,    41,    60,   287,    41,
     170,    46,    26,   210,   187,   294,   208,   208,   297,   211,
     211,    86,   608,   133,   303,   304,    41,    17,     0,   212,
      74,   347,   213,   210,    99,   100,   101,   102,   210,    74,
     212,   214,    74,    27,   109,   187,   609,   143,    62,   444,
     103,   338,   671,   696,   341,   641,   208,    99,    28,    74,
     156,   456,   208,    33,   341,   187,   187,   209,    52,   187,
     208,   350,   351,   187,   438,   213,   226,     9,   226,   366,
     367,   368,    66,   646,   149,   129,   672,    71,     3,   154,
     367,   368,   214,   214,   129,   187,   214,   129,   213,     3,
     214,     5,   252,   253,   252,   253,   385,   386,   387,   388,
     389,   390,   391,   157,   129,   678,    34,   209,     3,   213,
       5,    39,   157,   273,   211,   157,   234,   443,     6,     7,
     212,   213,   282,   212,   213,   480,   208,   151,   167,   212,
     213,   213,   157,   218,   219,   212,   213,   197,   198,   199,
      15,    16,    17,   197,   210,   214,   208,   207,   208,   207,
     208,   213,   197,   295,   208,   197,     3,   302,     5,   213,
     305,   185,   307,   308,   585,   103,   502,   312,   213,   212,
     525,   213,   197,   260,   463,   212,   260,   208,    41,   212,
     211,   586,     3,   208,     5,   327,   212,   347,   213,   347,
     182,   183,   184,   185,   186,   195,   196,   197,   198,   199,
     555,   210,   212,   295,     3,   210,     5,   207,   208,   212,
     352,    74,   548,   212,   503,   504,   189,   506,   507,   508,
     509,   510,   511,   512,   513,   514,   515,   516,   517,   518,
     519,   520,   521,   522,     3,   524,     5,   640,   591,   210,
      41,   212,   210,   210,   212,    46,   651,   212,     3,   391,
       5,   577,   212,     3,   346,     5,   346,   346,   212,   210,
     352,   550,     3,   346,     5,     9,   129,   409,   671,   346,
     606,   212,     3,    74,     5,   611,   368,     3,   614,     5,
     683,     3,     3,   443,     5,   443,   540,   692,   210,     3,
     544,     5,   213,   212,   157,   214,   213,   213,   701,    15,
      16,    17,    18,    19,    20,    21,    22,    23,   450,   213,
     213,   213,   213,   213,   213,   213,   458,   213,   208,   479,
     195,   196,   197,   198,   199,   694,   213,   653,   129,   213,
     213,   213,   207,   208,   197,   671,   213,   213,     5,   213,
     213,   213,   484,   213,   502,   208,   213,   213,   213,   213,
     213,   213,   494,   213,   213,   447,   157,   447,   447,   213,
     452,   450,   717,   455,   447,   455,   455,   447,   447,   458,
     447,   445,   455,   447,   447,   458,   447,   447,   455,   447,
     213,   458,   213,   213,   542,   213,   640,   210,   677,   643,
     212,   680,   212,   212,   210,   189,   197,   215,   213,   213,
     542,   484,   214,   214,     9,   694,     9,   208,     9,     9,
       9,     9,   213,     9,     9,     9,   208,   577,   213,   577,
     674,   212,   210,   210,   189,   722,   723,   212,   210,   683,
     214,   210,   215,   211,   187,   570,     6,   214,   214,   581,
     214,   213,   187,   603,   214,     9,   214,   701,   606,   591,
     542,   609,   214,   611,   214,   214,   214,   214,   214,    93,
     214,   167,   214,   214,   211,   214,   214,   609,   214,   207,
     189,   214,   188,   212,   190,   191,   192,   193,   194,   195,
     196,   197,   198,   199,   214,   214,   214,   214,   187,   214,
     214,   207,   208,   653,   214,   653,   214,   214,   214,   591,
     212,   214,   212,     3,   646,     3,     9,     6,   187,   214,
     212,   208,   212,   671,     5,   211,   214,   609,     3,   214,
       5,     6,     7,     8,     9,    10,    11,   214,   214,   189,
     210,   214,   214,   214,   212,   189,   678,     3,   214,   214,
       6,     7,     8,     9,    10,    11,   189,   212,   454,    34,
     339,    36,   479,   550,   646,   723,    53,    42,   452,   696,
     652,    46,   226,   652,   656,   299,   656,   656,   303,   652,
      82,   594,   309,   656,   596,   652,   356,   592,    63,   656,
      -1,    -1,    -1,    -1,    69,    70,   678,    72,    73,    -1,
      -1,    76,    -1,    -1,    79,    -1,    -1,    15,    16,    17,
      -1,    86,    20,    21,    -1,    -1,    -1,    73,    -1,    -1,
      -1,    -1,    97,    -1,    99,   100,   101,   102,    -1,    -1,
      -1,    -1,     3,    -1,   109,     6,     7,     8,     9,    10,
      11,    -1,    -1,    -1,    -1,    -1,   121,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,     3,    -1,    -1,     6,     7,     8,
       9,    10,    11,    -1,    -1,   121,    -1,    -1,   143,    -1,
      -1,    -1,    -1,   148,   149,   150,    -1,    -1,   153,   154,
      -1,   156,    -1,    -1,    -1,   160,    -1,    -1,    -1,    -1,
      -1,   166,    -1,    -1,   169,    -1,   171,   153,    -1,    -1,
      -1,    -1,    73,    -1,   179,   180,    -1,    -1,    -1,    -1,
     166,    -1,    -1,    -1,    -1,    -1,    -1,   192,    -1,    -1,
     195,   196,   197,    -1,    73,   200,   201,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,   192,    -1,   213,   195,
     196,   197,    -1,    -1,   200,   201,    -1,    -1,    -1,    -1,
     121,    -1,    -1,   209,    -1,    -1,    -1,   213,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   121,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   153,    -1,    -1,   193,   194,   195,   196,   197,
     198,   199,    -1,    -1,    -1,   166,    -1,    -1,    -1,   207,
     208,    -1,     4,     5,   153,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   166,    -1,    -1,
      -1,   192,    -1,    -1,   195,   196,   197,    -1,    -1,   200,
     201,    -1,    34,    -1,    36,    -1,    -1,    -1,   209,    -1,
      42,    43,   213,   192,    46,    -1,   195,   196,   197,    51,
      -1,   200,   201,    -1,    -1,    -1,    -1,    -1,    60,    -1,
      -1,    63,    -1,    -1,   213,    -1,    -1,    69,    70,    -1,
      72,    -1,    -1,    -1,    76,    -1,    -1,    79,    -1,    -1,
      -1,    -1,    -1,    -1,    86,    -1,    -1,    -1,    -1,    -1,
      -1,    93,    94,    -1,    -1,    97,    -1,    99,   100,   101,
     102,   103,    -1,    -1,    -1,    -1,    -1,   109,    -1,    -1,
      -1,    -1,   114,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,     4,     5,    -1,    -1,    -1,    -1,   131,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   143,    -1,    -1,    -1,    -1,   148,   149,   150,    -1,
      -1,    -1,   154,    34,   156,    36,    -1,    -1,   160,    -1,
      -1,    42,    43,    -1,    -1,    46,    -1,   169,    -1,   171,
      51,    -1,    -1,    -1,    -1,    -1,    -1,   179,   180,    60,
      -1,    -1,    63,    -1,    -1,    -1,    -1,    -1,    69,    70,
      -1,    72,    -1,    -1,    -1,    76,    -1,    -1,    79,    -1,
      -1,    -1,    -1,    -1,    -1,    86,   208,    -1,    -1,   211,
      -1,    -1,    93,    94,    -1,    -1,    97,    -1,    99,   100,
     101,   102,   103,    -1,    -1,    -1,    -1,    -1,   109,    -1,
      -1,    -1,    -1,   114,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,     4,     5,    -1,    -1,    -1,    -1,
     131,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   143,    -1,    -1,    -1,    -1,   148,   149,   150,
      -1,    -1,    -1,   154,    34,   156,    36,    -1,    -1,   160,
      -1,    -1,    42,    43,    -1,    -1,    46,    -1,   169,    -1,
     171,    51,    -1,    -1,    -1,    -1,    -1,    -1,   179,   180,
      60,    -1,    -1,    63,    -1,    -1,    -1,    -1,    -1,    69,
      70,    -1,    72,    -1,    -1,    -1,    76,    -1,    -1,    79,
      -1,    -1,    -1,    -1,    -1,    -1,    86,   208,    -1,    -1,
     211,    -1,    -1,    93,    -1,    -1,    -1,    97,    -1,    99,
     100,   101,   102,   103,    -1,    -1,    -1,    -1,    -1,   109,
      -1,    -1,    -1,    -1,   114,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,     4,     5,    -1,    -1,    -1,    -1,    -1,
      -1,   131,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   143,    -1,    -1,    -1,    -1,   148,   149,
     150,    -1,    -1,    34,   154,    36,   156,    -1,    -1,    -1,
     160,    42,    -1,    -1,    -1,    46,    -1,    -1,    -1,   169,
      51,   171,    15,    16,    17,    18,    19,    20,    21,   179,
     180,    -1,    63,    -1,    -1,    -1,    -1,    -1,    69,    70,
      -1,    72,    -1,    -1,    -1,    76,    -1,    -1,    79,    -1,
      -1,    -1,    -1,    -1,    -1,    86,    -1,    -1,   208,    -1,
      -1,   211,    93,    -1,    -1,    -1,    97,    -1,    99,   100,
     101,   102,    -1,    -1,    -1,    -1,    -1,    -1,   109,    -1,
      -1,    -1,    15,    16,    17,    18,    19,    20,    21,    -1,
      -1,    -1,     4,     5,    -1,    -1,    -1,    -1,    -1,    -1,
     131,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   143,    -1,    -1,    -1,    -1,   148,   149,   150,
      -1,    -1,    34,   154,    36,   156,    -1,    -1,    -1,   160,
      42,    -1,    -1,    -1,    46,    -1,    -1,    -1,   169,    51,
     171,    15,    16,    17,    18,    19,    20,    21,   179,   180,
      -1,    63,    -1,    -1,    -1,    -1,    -1,    69,    70,    -1,
      72,    -1,    -1,    -1,    76,    -1,    -1,    79,    -1,    -1,
      -1,    -1,    -1,    -1,    86,    -1,    -1,   208,    -1,    -1,
     211,    93,    -1,    -1,    -1,    97,    -1,    99,   100,   101,
     102,    -1,    -1,    -1,    -1,    -1,    -1,   109,   191,   192,
     193,   194,   195,   196,   197,   198,   199,    -1,    -1,    -1,
      -1,     4,     5,    -1,   207,   208,    -1,    -1,    -1,   131,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   143,    -1,    -1,    -1,    -1,   148,   149,   150,    -1,
      -1,    34,   154,    36,   156,    -1,    -1,    -1,   160,    42,
      -1,    -1,    -1,    46,    -1,    -1,    -1,   169,    51,   171,
     193,   194,   195,   196,   197,   198,   199,   179,   180,    -1,
      63,    -1,    -1,    -1,   207,   208,    69,    70,    -1,    72,
      -1,    -1,    -1,    76,    -1,    -1,    79,    -1,    -1,    -1,
      -1,    -1,    -1,    86,    -1,    -1,   208,    -1,    -1,   211,
      93,    -1,    -1,    -1,    97,    -1,    99,   100,   101,   102,
       5,    -1,    -1,    -1,    -1,    -1,   109,    -1,   192,   193,
     194,   195,   196,   197,   198,   199,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   207,   208,    -1,    -1,    -1,   131,    34,
      -1,    36,    -1,    -1,    -1,    -1,    -1,    42,    -1,    -1,
     143,    46,    -1,    -1,    -1,   148,   149,   150,    -1,    -1,
      -1,   154,    -1,   156,    -1,    -1,    -1,   160,    63,    -1,
      -1,    -1,    -1,    -1,    69,    70,   169,    72,   171,    -1,
      -1,    76,    -1,    -1,    79,    -1,   179,   180,    -1,    -1,
      -1,    86,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    97,    -1,    99,   100,   101,   102,    -1,    -1,
      -1,    -1,    -1,    -1,   109,   208,    -1,    -1,   211,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
       4,     5,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      14,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   143,    -1,
      -1,    -1,    -1,   148,   149,   150,    -1,    -1,    -1,   154,
      34,   156,    36,    -1,    -1,   160,    -1,    -1,    42,    43,
      -1,    -1,    46,    -1,   169,    -1,   171,    51,    -1,    -1,
      -1,    -1,    -1,    -1,   179,   180,    60,    -1,    -1,    63,
      -1,    -1,    -1,    -1,    -1,    69,    70,    -1,    72,    -1,
      -1,    -1,    76,    -1,    -1,    79,    -1,    -1,    -1,    -1,
      -1,    -1,    86,   208,    -1,    -1,   211,    -1,    -1,    93,
      -1,    -1,    -1,    97,    -1,    99,   100,   101,   102,   103,
      -1,    -1,    -1,    -1,    -1,   109,    -1,    -1,    -1,    -1,
     114,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   131,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   143,
       4,     5,    -1,    -1,   148,   149,   150,    -1,    -1,    13,
     154,    -1,   156,    -1,    -1,    -1,   160,    -1,    -1,    -1,
      -1,    -1,    -1,   167,    -1,   169,    -1,   171,    -1,    -1,
      34,    -1,    36,    -1,    -1,   179,   180,    -1,    42,    43,
      -1,    -1,    46,    -1,    -1,    -1,    -1,    51,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    60,    -1,    -1,    63,
      -1,    -1,    -1,    -1,   208,    69,    70,    -1,    72,    -1,
      -1,    -1,    76,    -1,    -1,    79,    -1,    -1,    -1,    -1,
      -1,    -1,    86,    -1,    -1,    -1,    -1,    -1,    -1,    93,
      94,    -1,    -1,    97,    -1,    99,   100,   101,   102,   103,
      -1,    -1,    -1,    -1,    -1,   109,    -1,    -1,    -1,    -1,
     114,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,     5,    -1,    -1,    -1,    -1,   131,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   143,
      -1,    -1,    -1,    -1,   148,   149,   150,    -1,    -1,    -1,
     154,    34,   156,    36,    -1,    -1,   160,    -1,    41,    42,
      -1,    -1,    -1,    46,    -1,   169,    -1,   171,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   179,   180,    -1,    -1,    -1,
      63,    -1,    -1,    -1,    -1,    -1,    69,    70,    -1,    72,
      -1,    74,    -1,    76,    -1,    -1,    79,    -1,    -1,    -1,
      -1,    -1,    -1,    86,   208,    -1,    -1,    -1,    -1,    -1,
      -1,     5,    -1,    -1,    97,    -1,    99,   100,   101,   102,
      -1,    -1,    -1,    -1,    -1,    -1,   109,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      34,    -1,    36,    -1,    -1,    -1,   129,    41,    42,    -1,
      -1,    -1,    46,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
     143,    -1,    -1,    -1,    -1,   148,   149,   150,    -1,    63,
      -1,   154,    -1,   156,   157,    69,    70,   160,    72,    -1,
      74,    -1,    76,    -1,    -1,    79,   169,    -1,   171,    -1,
      -1,    -1,    86,    -1,    -1,    -1,   179,   180,    -1,    -1,
       5,    -1,    -1,    97,    -1,    99,   100,   101,   102,    -1,
      -1,    -1,    -1,    -1,   197,   109,    -1,    -1,    -1,    24,
      -1,    -1,    -1,    -1,    -1,   208,    -1,    -1,    -1,    34,
      -1,    36,    -1,    -1,    -1,   129,    -1,    42,    -1,    -1,
      -1,    46,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   143,
      -1,    -1,    -1,    -1,   148,   149,   150,    -1,    63,    -1,
     154,    -1,   156,   157,    69,    70,   160,    72,    -1,    -1,
      -1,    76,    -1,    -1,    79,   169,    -1,   171,    -1,    -1,
      -1,    86,    -1,    -1,    -1,   179,   180,    -1,    -1,     5,
      -1,    -1,    97,    -1,    99,   100,   101,   102,    -1,    -1,
      -1,    -1,    -1,   197,   109,    15,    16,    17,    18,    19,
      20,    21,    22,    23,   208,    -1,    -1,    -1,    34,    -1,
      36,    -1,    -1,    -1,    -1,    -1,    42,    -1,    -1,    -1,
      46,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   143,    -1,
      -1,    -1,    -1,   148,   149,   150,    -1,    63,    -1,   154,
      -1,   156,    -1,    69,    70,   160,    72,    -1,    -1,    -1,
      76,    -1,    -1,    79,   169,    -1,   171,    -1,    -1,    -1,
      86,    -1,    -1,    -1,   179,   180,    -1,    -1,    -1,    -1,
      -1,    97,    -1,    99,   100,   101,   102,    -1,    -1,    -1,
      -1,    -1,    -1,   109,    15,    16,    17,    18,    19,    20,
      21,    22,    23,   208,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      15,    16,    17,    18,    19,    20,    21,   143,    23,    -1,
      -1,    -1,   148,   149,   150,    -1,    -1,    -1,   154,    -1,
     156,    -1,    -1,    -1,   160,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   169,    -1,   171,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,   179,   180,    15,    16,    17,    18,    19,
      20,    21,    22,    23,    -1,    -1,    -1,    -1,   188,    -1,
     190,   191,   192,   193,   194,   195,   196,   197,   198,   199,
      -1,    -1,   208,    -1,    -1,    -1,    -1,   207,   208,    -1,
      -1,    -1,    -1,    -1,   214,    15,    16,    17,    18,    19,
      20,    21,    22,    23,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    15,    16,    17,
      18,    19,    20,    21,    22,    23,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    15,    16,
      17,    18,    19,    20,    21,    22,    23,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   188,    -1,   190,
     191,   192,   193,   194,   195,   196,   197,   198,   199,    15,
      16,    17,    18,    19,    20,    21,   207,   208,    -1,    -1,
      -1,    -1,    -1,   214,    -1,   190,   191,   192,   193,   194,
     195,   196,   197,   198,   199,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   207,   208,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   188,    -1,
     190,   191,   192,   193,   194,   195,   196,   197,   198,   199,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   207,   208,   209,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   188,    -1,
     190,   191,   192,   193,   194,   195,   196,   197,   198,   199,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,   207,   208,   209,
     188,   189,   190,   191,   192,   193,   194,   195,   196,   197,
     198,   199,    -1,    -1,    -1,    -1,    -1,    -1,    -1,   207,
     208,   188,    -1,   190,   191,   192,   193,   194,   195,   196,
     197,   198,   199,    -1,    -1,    -1,     5,    -1,    -1,    -1,
     207,   208,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   190,   191,   192,   193,   194,   195,
     196,   197,   198,   199,    -1,    34,    -1,    36,    -1,    -1,
      -1,   207,   208,    42,    43,    -1,    -1,    46,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    60,    -1,    -1,    63,    -1,    -1,    -1,    -1,    -1,
      69,    70,    -1,    72,    -1,    -1,    -1,    76,    -1,    -1,
      79,    -1,    -1,    -1,    -1,    -1,    -1,    86,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,     5,    -1,    -1,    97,    -1,
      99,   100,   101,   102,   103,    -1,    -1,   106,    -1,    -1,
     109,    -1,    -1,    -1,   113,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    34,    -1,    36,    -1,    -1,    -1,
      -1,    -1,    42,    -1,    -1,    -1,    46,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   143,    -1,    -1,    -1,    -1,   148,
     149,   150,    -1,    63,    -1,   154,    -1,   156,    -1,    69,
      70,   160,    72,    -1,    -1,    -1,    76,    -1,    -1,    79,
     169,     5,   171,    -1,    -1,    -1,    86,    -1,    -1,    -1,
     179,   180,    -1,    -1,    -1,    -1,    -1,    97,    -1,    99,
     100,   101,   102,    -1,    -1,    -1,    -1,    -1,    -1,   109,
      34,    -1,    36,    -1,    -1,    -1,    -1,    -1,    42,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    63,
      -1,    -1,    -1,   143,    -1,    69,    70,    -1,   148,   149,
     150,    -1,    76,    -1,   154,    79,   156,    -1,    -1,    -1,
     160,    -1,    86,    -1,    -1,    -1,    -1,    -1,    -1,   169,
      -1,   171,    -1,    -1,    -1,    99,   100,   101,   102,   179,
     180,    -1,    -1,    -1,    -1,   109,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,   148,   149,   150,    -1,    -1,    -1,
     154,    -1,    -1,    -1,    -1,    -1,   160,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   169,    -1,   171,    -1,    -1,
      -1,    -1,    -1,    -1,    25,   179,   180,    28,    29,    30,
      31,    32,    33,    -1,    35,    -1,    -1,    38,    -1,    40,
      -1,    -1,    -1,    44,    45,    -1,    47,    48,    49,    50,
      -1,    52,    53,    54,    55,    56,    57,    58,    59,    -1,
      61,    -1,    -1,    64,    65,    66,    67,    68,    -1,    -1,
      71,    -1,    -1,    -1,    75,    -1,    77,    78,    -1,    80,
      81,    82,    83,    84,    85,    -1,    87,    88,    89,    90,
      91,    92,    -1,    -1,    95,    -1,    -1,    98,    -1,    -1,
      -1,    -1,    -1,   104,   105,    -1,   107,   108,    -1,   110,
     111,    -1,    -1,    -1,   115,   116,   117,   118,   119,   120,
      -1,   122,   123,   124,   125,   126,   127,   128,    -1,   130,
      -1,   132,    -1,   134,   135,   136,   137,   138,   139,   140,
     141,   142,    -1,   144,   145,   146,   147,    -1,    -1,    -1,
      -1,   152,    -1,    -1,   155,    -1,    -1,   158,   159,    -1,
      -1,   162,   163,   164,   165,    -1,    -1,   168,    -1,   170,
      -1,   172,   173,   174,   175,   176,   177,   178,    -1,    -1,
     181
};

/* YYSTOS[STATE-NUM] -- The symbol kind of the accessing symbol of
   state STATE-NUM.  */
static const yytype_int16 yystos[] =
{
       0,   217,   219,     0,     4,     5,    14,    34,    36,    42,
      43,    46,    51,    60,    63,    69,    70,    72,    76,    79,
      86,    93,    97,    99,   100,   101,   102,   103,   109,   114,
     131,   143,   148,   149,   150,   154,   156,   160,   169,   171,
     179,   180,   208,   218,   225,   226,   228,   229,   230,   231,
     234,   235,   241,   242,   253,   267,   271,   273,   274,   275,
     276,   277,   280,   281,   284,   286,   287,   288,   289,   291,
     292,   293,   294,   295,   297,   299,   317,   318,   319,   320,
     208,   322,   325,   326,     3,     5,   213,     3,     5,     3,
       5,   269,    99,   272,     9,     3,     5,   272,     3,   213,
     213,   272,   273,     3,   269,     3,   269,   273,    25,    28,
      29,    30,    31,    32,    33,    35,    38,    40,    44,    45,
      47,    48,    49,    50,    52,    53,    54,    55,    56,    57,
      58,    59,    61,    64,    65,    66,    67,    68,    71,    75,
      77,    78,    80,    81,    82,    83,    84,    85,    87,    88,
      89,    90,    91,    92,    95,    98,   104,   105,   107,   108,
     110,   111,   115,   116,   117,   118,   119,   120,   122,   123,
     124,   125,   126,   127,   128,   130,   132,   134,   135,   136,
     137,   138,   139,   140,   141,   142,   144,   145,   146,   147,
     152,   155,   158,   159,   162,   163,   164,   165,   168,   170,
     172,   173,   174,   175,   176,   177,   178,   181,   243,   245,
     316,   212,   221,   221,   167,    69,   106,   113,   160,   169,
     233,   253,   274,   280,   286,   290,   297,   317,   320,   212,
     212,   210,   210,   212,   210,   212,   224,   212,   189,   285,
     210,   298,   299,   298,   298,     3,     5,    41,    74,   129,
     157,   197,   213,   247,   270,   300,   301,   314,   253,   317,
     318,   320,   298,   212,    27,    52,    66,    71,   327,   328,
     322,   103,     9,   210,   212,     3,   318,   210,   161,   210,
     213,   213,   213,   213,   213,   213,   213,   213,   213,   213,
     213,   213,   213,   213,   213,   213,   213,   213,   213,   213,
     213,   213,   213,   213,   213,   213,   213,   213,   213,   213,
     213,   213,   213,   187,   209,    13,    94,   225,   232,   235,
     274,   276,   277,   288,   289,   292,   211,   241,   242,     3,
       3,     5,     3,     5,     3,     3,   210,   314,   278,   220,
     133,   282,   286,     3,     5,   210,   223,   296,   300,   300,
     215,   208,   213,   240,   298,   213,   187,   209,     5,   214,
     250,   251,   252,   270,   189,   214,   260,   213,   263,     9,
       9,    12,   246,   270,     3,     6,     7,     8,     9,    10,
      11,    73,   121,   153,   166,   192,   195,   196,   197,   200,
     201,   213,   256,   257,   258,   256,   259,     9,     9,   244,
     259,   258,     9,     9,   258,     9,   258,   256,   239,   242,
     297,   258,   254,   255,   256,     9,   316,     9,   258,   318,
     254,   256,   318,   182,   183,   184,   185,   186,   315,   318,
     318,   246,     6,     7,   321,     9,   318,   245,   208,   213,
     212,   210,   224,   297,   211,   241,   279,   219,   189,   112,
     241,   265,   283,   212,   210,   223,   211,   225,   242,   295,
     300,   214,   259,   197,   209,   256,   236,   237,   238,   239,
      26,    62,   151,   185,   329,   330,   328,   210,   211,   187,
     215,     6,   227,   211,   241,   261,   265,   211,   242,   262,
     265,   214,   214,   214,   213,   256,   256,   256,   256,   256,
     256,   256,   297,    15,    16,    17,    18,    19,    20,    21,
      22,    23,   188,   190,   191,   192,   193,   194,   195,   196,
     197,   198,   199,   207,   208,   187,   214,   214,   214,   187,
     214,   214,   214,   214,   214,   214,   214,   214,   214,   214,
     297,   197,   213,   240,   247,   270,   306,   308,   309,   214,
     187,   214,   214,   214,   214,   187,   214,   214,   214,   214,
     214,   214,   214,   214,   207,   214,   214,   214,   245,     9,
     222,   300,   310,   224,   289,   211,   189,   297,   317,   212,
     211,   242,   266,   267,   211,   231,   211,   224,   209,   209,
     214,   187,   187,   214,   167,   323,   324,   252,   258,     6,
     214,   297,   320,   214,   212,   212,   297,   214,   197,   213,
     240,   247,   302,   304,   305,   256,   256,     3,   256,   256,
     256,   256,   256,   256,   256,   256,   256,   256,   256,   256,
     256,   256,   256,   256,   256,     3,   256,   258,     9,   308,
     296,   197,   236,   247,   307,   308,   213,   240,   255,   258,
       6,   214,   221,   187,   300,   212,   223,   224,    24,   239,
     330,   326,   211,   323,   306,   312,   313,   212,   268,   270,
     304,   296,   197,   236,   247,   303,   304,   214,   213,   240,
     189,   209,   308,   296,   214,   308,   214,   236,   214,   224,
     211,   300,   211,     5,   189,   311,   187,   212,   210,   214,
     304,   296,   214,   308,   214,   256,   236,   256,   308,   214,
     224,   212,   259,   312,   248,   308,   214,    40,    53,   211,
     249,   258,   189,   189,   212,   264,   265,   264,   212
};

/* YYR1[RULE-NUM] -- Symbol kind of the left-hand side of rule RULE-NUM.  */
static const yytype_int16 yyr1[] =
{
       0,   216,   217,   218,   218,   219,   220,   219,   219,   219,
     219,   219,   219,   219,   219,   221,   221,   222,   221,   221,
     221,   221,   221,   221,   221,   221,   223,   223,   224,   224,
     225,   225,   225,   225,   225,   225,   225,   226,   227,   227,
     228,   228,   228,   228,   228,   228,   228,   228,   228,   229,
     230,   231,   232,   233,   233,   234,   235,   236,   236,   237,
     237,   238,   238,   239,   239,   240,   240,   240,   241,   241,
     242,   243,   243,   243,   244,   244,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   245,   245,   245,   245,
     245,   245,   245,   245,   245,   245,   246,   246,   247,   247,
     247,   247,   248,   248,   249,   249,   250,   250,   250,   251,
     251,   252,   252,   253,   254,   254,   255,   255,   256,   256,
     256,   256,   256,   256,   256,   256,   256,   256,   256,   256,
     256,   256,   256,   256,   256,   256,   256,   256,   256,   256,
     256,   256,   256,   256,   256,   256,   256,   256,   256,   256,
     256,   256,   256,   256,   256,   256,   256,   256,   256,   257,
     257,   258,   259,   260,   260,   261,   261,   262,   262,   263,
     263,   264,   264,   265,   265,   266,   267,   267,   268,   268,
     269,   269,   269,   270,   270,   271,   271,   271,   271,   271,
     271,   271,   271,   271,   271,   271,   272,   272,   273,   273,
     273,   273,   273,   273,   273,   273,   273,   274,   274,   275,
     276,   277,   278,   278,   279,   280,   280,   281,   282,   282,
     283,   283,   284,   284,   285,   285,   286,   286,   287,   288,
     288,   288,   289,   289,   290,   290,   291,   292,   293,   293,
     293,   294,   295,   296,   296,   297,   297,   298,   298,   299,
     299,   299,   300,   300,   300,   301,   301,   301,   301,   302,
     302,   302,   303,   303,   304,   304,   305,   305,   305,   305,
     305,   306,   306,   306,   307,   307,   308,   308,   309,   309,
     309,   309,   309,   309,   310,   310,   311,   311,   312,   313,
     313,   314,   314,   315,   315,   315,   315,   315,   316,   316,
     316,   317,   318,   318,   318,   318,   318,   318,   318,   318,
     318,   318,   319,   320,   320,   321,   321,   321,   322,   322,
     323,   323,   324,   325,   326,   326,   327,   327,   328,   328,
     328,   328,   329,   329,   330,   330,   330,   330
};

/* YYR2[RULE-NUM] -- Number of symbols on the right-hand side of rule RULE-NUM.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     2,     0,     2,     0,     0,     6,     2,     2,
       3,     2,     2,     2,     2,     0,     2,     0,     6,     2,
       3,     2,     2,     2,     2,     2,     0,     2,     0,     1,
       1,     2,     2,     1,     2,     1,     1,     6,     1,     2,
       1,     2,     1,     2,     1,     2,     2,     2,     2,     4,
       3,     3,     5,     2,     2,     3,     4,     0,     1,     1,
       3,     1,     3,     3,     2,     3,     3,     2,     0,     1,
       3,     1,     3,     4,     1,     3,     0,     1,     4,     1,
       1,     1,     1,     1,     4,     4,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     4,     1,     1,     1,
       4,     1,     1,     1,     4,     4,     1,     1,     1,     1,
       4,     4,     4,     4,     4,     1,     4,     1,     1,     4,
       1,     4,     1,     1,     4,     4,     1,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     1,     1,
       4,     1,     1,     1,     4,     4,     1,     1,     1,     1,
       1,     6,     1,     4,     1,     1,     1,     4,     1,     1,
       1,     4,     4,     4,     4,     1,     1,     4,     4,     4,
       1,     1,     4,     4,     4,     1,     1,     1,     1,     1,
       1,     1,     0,     2,     4,     3,     0,     2,     1,     1,
       3,     3,     1,     5,     1,     3,     0,     1,     1,     1,
       1,     1,     1,     1,     1,     1,     1,     1,     5,     3,
       3,     3,     3,     3,     3,     3,     3,     3,     3,     3,
       3,     3,     3,     3,     3,     3,     3,     2,     2,     2,
       2,     2,     2,     3,     3,     5,     5,     4,     3,     1,
       3,     1,     1,     0,     2,     4,     3,     2,     2,     0,
       2,     2,     1,     3,     2,     1,     3,     2,     0,     1,
       0,     1,     1,     1,     1,     1,     1,     1,     2,     2,
       1,     1,     1,     1,     1,     1,     0,     1,     1,     2,
       1,     2,     2,     1,     1,     1,     1,     2,     2,     2,
       5,     2,     0,     2,     2,     2,     2,     2,     2,     3,
       2,     3,     5,     5,     0,     2,     2,     2,     2,     6,
       8,     2,     2,     2,     2,     2,     2,     5,     1,     1,
       1,     1,     1,     0,     2,     2,     3,     0,     1,     2,
       2,     2,     3,     2,     1,     1,     3,     2,     4,     3,
       2,     1,     3,     2,     0,     1,     3,     2,     1,     3,
       4,     3,     2,     1,     3,     2,     0,     1,     1,     3,
       2,     1,     3,     4,     1,     3,     0,     2,     2,     1,
       3,     1,     3,     1,     1,     1,     1,     1,     1,     1,
       1,     5,     1,     1,     1,     1,     2,     1,     2,     1,
       2,     4,     5,     5,    10,     1,     3,     1,     0,     2,
       0,     2,     4,     6,     0,     3,     1,     3,     4,     1,
       1,     1,     1,     3,     1,     1,     1,     1
};


enum { YYENOMEM = -2 };

#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = YYEMPTY)

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab
#define YYNOMEM         goto yyexhaustedlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                    \
  do                                                              \
    if (yychar == YYEMPTY)                                        \
      {                                                           \
        yychar = (Token);                                         \
        yylval = (Value);                                         \
        YYPOPSTACK (yylen);                                       \
        yystate = *yyssp;                                         \
        goto yybackup;                                            \
      }                                                           \
    else                                                          \
      {                                                           \
        yyerror (YY_("syntax error: cannot back up")); \
        YYERROR;                                                  \
      }                                                           \
  while (0)

/* Backward compatibility with an undocumented macro.
   Use YYerror or YYUNDEF. */
#define YYERRCODE YYUNDEF


/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)




# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Kind, Value); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*-----------------------------------.
| Print this symbol's value on YYO.  |
`-----------------------------------*/

static void
yy_symbol_value_print (FILE *yyo,
                       yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep)
{
  FILE *yyoutput = yyo;
  YY_USE (yyoutput);
  if (!yyvaluep)
    return;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YY_USE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/*---------------------------.
| Print this symbol on YYO.  |
`---------------------------*/

static void
yy_symbol_print (FILE *yyo,
                 yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep)
{
  YYFPRINTF (yyo, "%s %s (",
             yykind < YYNTOKENS ? "token" : "nterm", yysymbol_name (yykind));

  yy_symbol_value_print (yyo, yykind, yyvaluep);
  YYFPRINTF (yyo, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yy_state_t *yybottom, yy_state_t *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yy_state_t *yyssp, YYSTYPE *yyvsp,
                 int yyrule)
{
  int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %d):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       YY_ACCESSING_SYMBOL (+yyssp[yyi + 1 - yynrhs]),
                       &yyvsp[(yyi + 1) - (yynrhs)]);
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, Rule); \
} while (0)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args) ((void) 0)
# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif


/* Context of a parse error.  */
typedef struct
{
  yy_state_t *yyssp;
  yysymbol_kind_t yytoken;
} yypcontext_t;

/* Put in YYARG at most YYARGN of the expected tokens given the
   current YYCTX, and return the number of tokens stored in YYARG.  If
   YYARG is null, return the number of expected tokens (guaranteed to
   be less than YYNTOKENS).  Return YYENOMEM on memory exhaustion.
   Return 0 if there are more than YYARGN expected tokens, yet fill
   YYARG up to YYARGN. */
static int
yypcontext_expected_tokens (const yypcontext_t *yyctx,
                            yysymbol_kind_t yyarg[], int yyargn)
{
  /* Actual size of YYARG. */
  int yycount = 0;
  int yyn = yypact[+*yyctx->yyssp];
  if (!yypact_value_is_default (yyn))
    {
      /* Start YYX at -YYN if negative to avoid negative indexes in
         YYCHECK.  In other words, skip the first -YYN actions for
         this state because they are default actions.  */
      int yyxbegin = yyn < 0 ? -yyn : 0;
      /* Stay within bounds of both yycheck and yytname.  */
      int yychecklim = YYLAST - yyn + 1;
      int yyxend = yychecklim < YYNTOKENS ? yychecklim : YYNTOKENS;
      int yyx;
      for (yyx = yyxbegin; yyx < yyxend; ++yyx)
        if (yycheck[yyx + yyn] == yyx && yyx != YYSYMBOL_YYerror
            && !yytable_value_is_error (yytable[yyx + yyn]))
          {
            if (!yyarg)
              ++yycount;
            else if (yycount == yyargn)
              return 0;
            else
              yyarg[yycount++] = YY_CAST (yysymbol_kind_t, yyx);
          }
    }
  if (yyarg && yycount == 0 && 0 < yyargn)
    yyarg[0] = YYSYMBOL_YYEMPTY;
  return yycount;
}




#ifndef yystrlen
# if defined __GLIBC__ && defined _STRING_H
#  define yystrlen(S) (YY_CAST (YYPTRDIFF_T, strlen (S)))
# else
/* Return the length of YYSTR.  */
static YYPTRDIFF_T
yystrlen (const char *yystr)
{
  YYPTRDIFF_T yylen;
  for (yylen = 0; yystr[yylen]; yylen++)
    continue;
  return yylen;
}
# endif
#endif

#ifndef yystpcpy
# if defined __GLIBC__ && defined _STRING_H && defined _GNU_SOURCE
#  define yystpcpy stpcpy
# else
/* Copy YYSRC to YYDEST, returning the address of the terminating '\0' in
   YYDEST.  */
static char *
yystpcpy (char *yydest, const char *yysrc)
{
  char *yyd = yydest;
  const char *yys = yysrc;

  while ((*yyd++ = *yys++) != '\0')
    continue;

  return yyd - 1;
}
# endif
#endif

#ifndef yytnamerr
/* Copy to YYRES the contents of YYSTR after stripping away unnecessary
   quotes and backslashes, so that it's suitable for yyerror.  The
   heuristic is that double-quoting is unnecessary unless the string
   contains an apostrophe, a comma, or backslash (other than
   backslash-backslash).  YYSTR is taken from yytname.  If YYRES is
   null, do not copy; instead, return the length of what the result
   would have been.  */
static YYPTRDIFF_T
yytnamerr (char *yyres, const char *yystr)
{
  if (*yystr == '"')
    {
      YYPTRDIFF_T yyn = 0;
      char const *yyp = yystr;
      for (;;)
        switch (*++yyp)
          {
          case '\'':
          case ',':
            goto do_not_strip_quotes;

          case '\\':
            if (*++yyp != '\\')
              goto do_not_strip_quotes;
            else
              goto append;

          append:
          default:
            if (yyres)
              yyres[yyn] = *yyp;
            yyn++;
            break;

          case '"':
            if (yyres)
              yyres[yyn] = '\0';
            return yyn;
          }
    do_not_strip_quotes: ;
    }

  if (yyres)
    return yystpcpy (yyres, yystr) - yyres;
  else
    return yystrlen (yystr);
}
#endif


static int
yy_syntax_error_arguments (const yypcontext_t *yyctx,
                           yysymbol_kind_t yyarg[], int yyargn)
{
  /* Actual size of YYARG. */
  int yycount = 0;
  /* There are many possibilities here to consider:
     - If this state is a consistent state with a default action, then
       the only way this function was invoked is if the default action
       is an error action.  In that case, don't check for expected
       tokens because there are none.
     - The only way there can be no lookahead present (in yychar) is if
       this state is a consistent state with a default action.  Thus,
       detecting the absence of a lookahead is sufficient to determine
       that there is no unexpected or expected token to report.  In that
       case, just report a simple "syntax error".
     - Don't assume there isn't a lookahead just because this state is a
       consistent state with a default action.  There might have been a
       previous inconsistent state, consistent state with a non-default
       action, or user semantic action that manipulated yychar.
     - Of course, the expected token list depends on states to have
       correct lookahead information, and it depends on the parser not
       to perform extra reductions after fetching a lookahead from the
       scanner and before detecting a syntax error.  Thus, state merging
       (from LALR or IELR) and default reductions corrupt the expected
       token list.  However, the list is correct for canonical LR with
       one exception: it will still contain any token that will not be
       accepted due to an error action in a later state.
  */
  if (yyctx->yytoken != YYSYMBOL_YYEMPTY)
    {
      int yyn;
      if (yyarg)
        yyarg[yycount] = yyctx->yytoken;
      ++yycount;
      yyn = yypcontext_expected_tokens (yyctx,
                                        yyarg ? yyarg + 1 : yyarg, yyargn - 1);
      if (yyn == YYENOMEM)
        return YYENOMEM;
      else
        yycount += yyn;
    }
  return yycount;
}

/* Copy into *YYMSG, which is of size *YYMSG_ALLOC, an error message
   about the unexpected token YYTOKEN for the state stack whose top is
   YYSSP.

   Return 0 if *YYMSG was successfully written.  Return -1 if *YYMSG is
   not large enough to hold the message.  In that case, also set
   *YYMSG_ALLOC to the required number of bytes.  Return YYENOMEM if the
   required number of bytes is too large to store.  */
static int
yysyntax_error (YYPTRDIFF_T *yymsg_alloc, char **yymsg,
                const yypcontext_t *yyctx)
{
  enum { YYARGS_MAX = 5 };
  /* Internationalized format string. */
  const char *yyformat = YY_NULLPTR;
  /* Arguments of yyformat: reported tokens (one for the "unexpected",
     one per "expected"). */
  yysymbol_kind_t yyarg[YYARGS_MAX];
  /* Cumulated lengths of YYARG.  */
  YYPTRDIFF_T yysize = 0;

  /* Actual size of YYARG. */
  int yycount = yy_syntax_error_arguments (yyctx, yyarg, YYARGS_MAX);
  if (yycount == YYENOMEM)
    return YYENOMEM;

  switch (yycount)
    {
#define YYCASE_(N, S)                       \
      case N:                               \
        yyformat = S;                       \
        break
    default: /* Avoid compiler warnings. */
      YYCASE_(0, YY_("syntax error"));
      YYCASE_(1, YY_("syntax error, unexpected %s"));
      YYCASE_(2, YY_("syntax error, unexpected %s, expecting %s"));
      YYCASE_(3, YY_("syntax error, unexpected %s, expecting %s or %s"));
      YYCASE_(4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
      YYCASE_(5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
#undef YYCASE_
    }

  /* Compute error message size.  Don't count the "%s"s, but reserve
     room for the terminator.  */
  yysize = yystrlen (yyformat) - 2 * yycount + 1;
  {
    int yyi;
    for (yyi = 0; yyi < yycount; ++yyi)
      {
        YYPTRDIFF_T yysize1
          = yysize + yytnamerr (YY_NULLPTR, yytname[yyarg[yyi]]);
        if (yysize <= yysize1 && yysize1 <= YYSTACK_ALLOC_MAXIMUM)
          yysize = yysize1;
        else
          return YYENOMEM;
      }
  }

  if (*yymsg_alloc < yysize)
    {
      *yymsg_alloc = 2 * yysize;
      if (! (yysize <= *yymsg_alloc
             && *yymsg_alloc <= YYSTACK_ALLOC_MAXIMUM))
        *yymsg_alloc = YYSTACK_ALLOC_MAXIMUM;
      return -1;
    }

  /* Avoid sprintf, as that infringes on the user's name space.
     Don't have undefined behavior even if the translation
     produced a string with the wrong number of "%s"s.  */
  {
    char *yyp = *yymsg;
    int yyi = 0;
    while ((*yyp = *yyformat) != '\0')
      if (*yyp == '%' && yyformat[1] == 's' && yyi < yycount)
        {
          yyp += yytnamerr (yyp, yytname[yyarg[yyi++]]);
          yyformat += 2;
        }
      else
        {
          ++yyp;
          ++yyformat;
        }
  }
  return 0;
}


/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg,
            yysymbol_kind_t yykind, YYSTYPE *yyvaluep)
{
  YY_USE (yyvaluep);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yykind, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YY_USE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/* Lookahead token kind.  */
int yychar;

/* The semantic value of the lookahead symbol.  */
YYSTYPE yylval;
/* Number of syntax errors so far.  */
int yynerrs;




/*----------.
| yyparse.  |
`----------*/

int
yyparse (void)
{
    yy_state_fast_t yystate = 0;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus = 0;

    /* Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* Their size.  */
    YYPTRDIFF_T yystacksize = YYINITDEPTH;

    /* The state stack: array, bottom, top.  */
    yy_state_t yyssa[YYINITDEPTH];
    yy_state_t *yyss = yyssa;
    yy_state_t *yyssp = yyss;

    /* The semantic value stack: array, bottom, top.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs = yyvsa;
    YYSTYPE *yyvsp = yyvs;

  int yyn;
  /* The return value of yyparse.  */
  int yyresult;
  /* Lookahead symbol kind.  */
  yysymbol_kind_t yytoken = YYSYMBOL_YYEMPTY;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;

  /* Buffer for error messages, and its allocated size.  */
  char yymsgbuf[128];
  char *yymsg = yymsgbuf;
  YYPTRDIFF_T yymsg_alloc = sizeof yymsgbuf;

#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yychar = YYEMPTY; /* Cause a token to be read.  */

  goto yysetstate;


/*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;


/*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
yysetstate:
  YYDPRINTF ((stderr, "Entering state %d\n", yystate));
  YY_ASSERT (0 <= yystate && yystate < YYNSTATES);
  YY_IGNORE_USELESS_CAST_BEGIN
  *yyssp = YY_CAST (yy_state_t, yystate);
  YY_IGNORE_USELESS_CAST_END
  YY_STACK_PRINT (yyss, yyssp);

  if (yyss + yystacksize - 1 <= yyssp)
#if !defined yyoverflow && !defined YYSTACK_RELOCATE
    YYNOMEM;
#else
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYPTRDIFF_T yysize = yyssp - yyss + 1;

# if defined yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        yy_state_t *yyss1 = yyss;
        YYSTYPE *yyvs1 = yyvs;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * YYSIZEOF (*yyssp),
                    &yyvs1, yysize * YYSIZEOF (*yyvsp),
                    &yystacksize);
        yyss = yyss1;
        yyvs = yyvs1;
      }
# else /* defined YYSTACK_RELOCATE */
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        YYNOMEM;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yy_state_t *yyss1 = yyss;
        union yyalloc *yyptr =
          YY_CAST (union yyalloc *,
                   YYSTACK_ALLOC (YY_CAST (YYSIZE_T, YYSTACK_BYTES (yystacksize))));
        if (! yyptr)
          YYNOMEM;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
#  undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;

      YY_IGNORE_USELESS_CAST_BEGIN
      YYDPRINTF ((stderr, "Stack size increased to %ld\n",
                  YY_CAST (long, yystacksize)));
      YY_IGNORE_USELESS_CAST_END

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }
#endif /* !defined yyoverflow && !defined YYSTACK_RELOCATE */


  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;


/*-----------.
| yybackup.  |
`-----------*/
yybackup:
  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either empty, or end-of-input, or a valid lookahead.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token\n"));
      yychar = yylex ();
    }

  if (yychar <= YYEOF)
    {
      yychar = YYEOF;
      yytoken = YYSYMBOL_YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else if (yychar == YYerror)
    {
      /* The scanner already issued an error message, process directly
         to error recovery.  But do not keep the error token as
         lookahead, it is too special and may lead us to an endless
         loop in error recovery. */
      yychar = YYUNDEF;
      yytoken = YYSYMBOL_YYerror;
      goto yyerrlab1;
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);
  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  /* Discard the shifted token.  */
  yychar = YYEMPTY;
  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];


  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
  case 2: /* input: gbl_statements m_acf  */
#line 326 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { fix_incomplete();
						  check_statements((yyvsp[-1].stmt_list), FALSE);
						  check_all_user_types((yyvsp[-1].stmt_list));
						  write_header((yyvsp[-1].stmt_list));
						  write_id_data((yyvsp[-1].stmt_list));
						  write_proxies((yyvsp[-1].stmt_list));
						  write_client((yyvsp[-1].stmt_list));
						  write_server((yyvsp[-1].stmt_list));
						  write_regscript((yyvsp[-1].stmt_list));
#ifndef __REACTOS__
						  write_typelib_regscript((yyvsp[-1].stmt_list));
#endif
						  write_dlldata((yyvsp[-1].stmt_list));
						  write_local_stubs((yyvsp[-1].stmt_list));
						}
#line 2884 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 5: /* gbl_statements: %empty  */
#line 345 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = NULL; }
#line 2890 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 6: /* $@1: %empty  */
#line 346 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                          { push_namespace((yyvsp[-1].str)); }
#line 2896 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 7: /* gbl_statements: gbl_statements namespacedef '{' $@1 gbl_statements '}'  */
#line 347 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { pop_namespace((yyvsp[-4].str)); (yyval.stmt_list) = append_statements((yyvsp[-5].stmt_list), (yyvsp[-1].stmt_list)); }
#line 2902 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 8: /* gbl_statements: gbl_statements interfacedec  */
#line 348 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_reference((yyvsp[0].type))); }
#line 2908 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 9: /* gbl_statements: gbl_statements interfacedef  */
#line 349 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_type_decl((yyvsp[0].type))); }
#line 2914 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 10: /* gbl_statements: gbl_statements coclass ';'  */
#line 350 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = (yyvsp[-2].stmt_list);
						  reg_type((yyvsp[-1].type), (yyvsp[-1].type)->name, current_namespace, 0);
						}
#line 2922 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 11: /* gbl_statements: gbl_statements coclassdef  */
#line 353 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_type_decl((yyvsp[0].type)));
						  reg_type((yyvsp[0].type), (yyvsp[0].type)->name, current_namespace, 0);
						}
#line 2930 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 12: /* gbl_statements: gbl_statements moduledef  */
#line 356 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_module((yyvsp[0].type))); }
#line 2936 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 13: /* gbl_statements: gbl_statements librarydef  */
#line 357 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_library((yyvsp[0].typelib))); }
#line 2942 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 14: /* gbl_statements: gbl_statements statement  */
#line 358 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), (yyvsp[0].statement)); }
#line 2948 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 15: /* imp_statements: %empty  */
#line 361 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = NULL; }
#line 2954 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 16: /* imp_statements: imp_statements interfacedec  */
#line 362 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_reference((yyvsp[0].type))); }
#line 2960 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 17: /* $@2: %empty  */
#line 363 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                          { push_namespace((yyvsp[-1].str)); }
#line 2966 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 18: /* imp_statements: imp_statements namespacedef '{' $@2 imp_statements '}'  */
#line 364 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { pop_namespace((yyvsp[-4].str)); (yyval.stmt_list) = append_statements((yyvsp[-5].stmt_list), (yyvsp[-1].stmt_list)); }
#line 2972 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 19: /* imp_statements: imp_statements interfacedef  */
#line 365 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_type_decl((yyvsp[0].type))); }
#line 2978 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 20: /* imp_statements: imp_statements coclass ';'  */
#line 366 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = (yyvsp[-2].stmt_list); reg_type((yyvsp[-1].type), (yyvsp[-1].type)->name, current_namespace, 0); }
#line 2984 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 21: /* imp_statements: imp_statements coclassdef  */
#line 367 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_type_decl((yyvsp[0].type)));
						  reg_type((yyvsp[0].type), (yyvsp[0].type)->name, current_namespace, 0);
						}
#line 2992 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 22: /* imp_statements: imp_statements moduledef  */
#line 370 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_module((yyvsp[0].type))); }
#line 2998 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 23: /* imp_statements: imp_statements statement  */
#line 371 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), (yyvsp[0].statement)); }
#line 3004 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 24: /* imp_statements: imp_statements importlib  */
#line 372 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_importlib((yyvsp[0].str))); }
#line 3010 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 25: /* imp_statements: imp_statements librarydef  */
#line 373 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), make_statement_library((yyvsp[0].typelib))); }
#line 3016 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 26: /* int_statements: %empty  */
#line 376 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = NULL; }
#line 3022 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 27: /* int_statements: int_statements statement  */
#line 377 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stmt_list) = append_statement((yyvsp[-1].stmt_list), (yyvsp[0].statement)); }
#line 3028 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 30: /* statement: cppquote  */
#line 385 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.statement) = make_statement_cppquote((yyvsp[0].str)); }
#line 3034 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 31: /* statement: typedecl ';'  */
#line 386 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.statement) = make_statement_type_decl((yyvsp[-1].type)); }
#line 3040 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 32: /* statement: declaration ';'  */
#line 387 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.statement) = make_statement_declaration((yyvsp[-1].var)); }
#line 3046 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 33: /* statement: import  */
#line 388 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.statement) = make_statement_import((yyvsp[0].str)); }
#line 3052 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 34: /* statement: typedef ';'  */
#line 389 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.statement) = (yyvsp[-1].statement); }
#line 3058 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 35: /* statement: aPRAGMA  */
#line 390 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.statement) = make_statement_pragma((yyvsp[0].str)); }
#line 3064 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 36: /* statement: pragma_warning  */
#line 391 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                         { (yyval.statement) = NULL; }
#line 3070 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 37: /* pragma_warning: tPRAGMA_WARNING '(' aIDENTIFIER ':' warnings ')'  */
#line 395 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                  {
                      int result;
                      (yyval.statement) = NULL;
                      result = do_warning((yyvsp[-3].str), (yyvsp[-1].warning_list));
                      if(!result)
                          error_loc("expected \"disable\" or \"enable\"\n");
                  }
#line 3082 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 38: /* warnings: aNUM  */
#line 405 "/home/moebius/reactos/sdk/tools/widl/parser.y"
               { (yyval.warning_list) = append_warning(NULL, (yyvsp[0].num)); }
#line 3088 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 39: /* warnings: warnings aNUM  */
#line 406 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                        { (yyval.warning_list) = append_warning((yyvsp[-1].warning_list), (yyvsp[0].num)); }
#line 3094 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 41: /* typedecl: tENUM aIDENTIFIER  */
#line 411 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_enum((yyvsp[0].str), current_namespace, FALSE, NULL); }
#line 3100 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 43: /* typedecl: tSTRUCT aIDENTIFIER  */
#line 413 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_struct((yyvsp[0].str), current_namespace, FALSE, NULL); }
#line 3106 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 45: /* typedecl: tUNION aIDENTIFIER  */
#line 415 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_nonencapsulated_union((yyvsp[0].str), FALSE, NULL); }
#line 3112 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 46: /* typedecl: attributes enumdef  */
#line 416 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); (yyval.type)->attrs = check_enum_attrs((yyvsp[-1].attr_list)); }
#line 3118 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 47: /* typedecl: attributes structdef  */
#line 417 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); (yyval.type)->attrs = check_struct_attrs((yyvsp[-1].attr_list)); }
#line 3124 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 48: /* typedecl: attributes uniondef  */
#line 418 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); (yyval.type)->attrs = check_union_attrs((yyvsp[-1].attr_list)); }
#line 3130 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 49: /* cppquote: tCPPQUOTE '(' aSTRING ')'  */
#line 421 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[-1].str); }
#line 3136 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 50: /* import_start: tIMPORT aSTRING ';'  */
#line 423 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { assert(yychar == YYEMPTY);
						  (yyval.import) = xmalloc(sizeof(struct _import_t));
						  (yyval.import)->name = (yyvsp[-1].str);
						  (yyval.import)->import_performed = do_import((yyvsp[-1].str));
						  if (!(yyval.import)->import_performed) yychar = aEOF;
						}
#line 3147 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 51: /* import: import_start imp_statements aEOF  */
#line 431 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[-2].import)->name;
						  if ((yyvsp[-2].import)->import_performed) pop_import();
						  free((yyvsp[-2].import));
						}
#line 3156 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 52: /* importlib: tIMPORTLIB '(' aSTRING ')' semicolon_opt  */
#line 439 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[-2].str); if(!parse_only) add_importlib((yyvsp[-2].str)); }
#line 3162 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 53: /* libraryhdr: tLIBRARY aIDENTIFIER  */
#line 445 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[0].str); }
#line 3168 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 54: /* libraryhdr: tLIBRARY aKNOWNTYPE  */
#line 446 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[0].str); }
#line 3174 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 55: /* library_start: attributes libraryhdr '{'  */
#line 448 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.typelib) = make_library((yyvsp[-1].str), check_library_attrs((yyvsp[-1].str), (yyvsp[-2].attr_list)));
/* ifdef __REACTOS__ */
						  if (!parse_only) start_typelib((yyval.typelib));
/* else
						  if (!parse_only && do_typelib) current_typelib = $$;
*/
						}
#line 3186 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 56: /* librarydef: library_start imp_statements '}' semicolon_opt  */
#line 458 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.typelib) = (yyvsp[-3].typelib);
						  (yyval.typelib)->stmts = (yyvsp[-2].stmt_list);
						  if (!parse_only) end_typelib();
						}
#line 3195 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 57: /* m_args: %empty  */
#line 467 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 3201 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 59: /* arg_list: arg  */
#line 471 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { check_arg_attrs((yyvsp[0].var)); (yyval.var_list) = append_var( NULL, (yyvsp[0].var) ); }
#line 3207 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 60: /* arg_list: arg_list ',' arg  */
#line 472 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { check_arg_attrs((yyvsp[0].var)); (yyval.var_list) = append_var( (yyvsp[-2].var_list), (yyvsp[0].var) ); }
#line 3213 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 62: /* args: arg_list ',' ELLIPSIS  */
#line 476 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = append_var( (yyvsp[-2].var_list), make_var(strdup("...")) ); }
#line 3219 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 63: /* arg: attributes decl_spec m_any_declarator  */
#line 480 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { if ((yyvsp[-1].declspec)->stgclass != STG_NONE && (yyvsp[-1].declspec)->stgclass != STG_REGISTER)
						    error_loc("invalid storage class for function parameter\n");
						  (yyval.var) = declare_var((yyvsp[-2].attr_list), (yyvsp[-1].declspec), (yyvsp[0].declarator), TRUE);
						  free((yyvsp[-1].declspec)); free((yyvsp[0].declarator));
						}
#line 3229 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 64: /* arg: decl_spec m_any_declarator  */
#line 485 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { if ((yyvsp[-1].declspec)->stgclass != STG_NONE && (yyvsp[-1].declspec)->stgclass != STG_REGISTER)
						    error_loc("invalid storage class for function parameter\n");
						  (yyval.var) = declare_var(NULL, (yyvsp[-1].declspec), (yyvsp[0].declarator), TRUE);
						  free((yyvsp[-1].declspec)); free((yyvsp[0].declarator));
						}
#line 3239 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 65: /* array: '[' expr ']'  */
#line 492 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = (yyvsp[-1].expr);
						  if (!(yyval.expr)->is_const || (yyval.expr)->cval <= 0)
						      error_loc("array dimension is not a positive integer constant\n");
						}
#line 3248 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 66: /* array: '[' '*' ']'  */
#line 496 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr(EXPR_VOID); }
#line 3254 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 67: /* array: '[' ']'  */
#line 497 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr(EXPR_VOID); }
#line 3260 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 68: /* m_attributes: %empty  */
#line 500 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = NULL; }
#line 3266 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 70: /* attributes: '[' attrib_list ']'  */
#line 505 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = (yyvsp[-1].attr_list); }
#line 3272 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 71: /* attrib_list: attribute  */
#line 508 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = append_attr( NULL, (yyvsp[0].attr) ); }
#line 3278 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 72: /* attrib_list: attrib_list ',' attribute  */
#line 509 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = append_attr( (yyvsp[-2].attr_list), (yyvsp[0].attr) ); }
#line 3284 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 73: /* attrib_list: attrib_list ']' '[' attribute  */
#line 510 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = append_attr( (yyvsp[-3].attr_list), (yyvsp[0].attr) ); }
#line 3290 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 74: /* str_list: aSTRING  */
#line 513 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str_list) = append_str( NULL, (yyvsp[0].str) ); }
#line 3296 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 75: /* str_list: str_list ',' aSTRING  */
#line 514 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str_list) = append_str( (yyvsp[-2].str_list), (yyvsp[0].str) ); }
#line 3302 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 76: /* attribute: %empty  */
#line 517 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = NULL; }
#line 3308 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 77: /* attribute: tAGGREGATABLE  */
#line 518 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_AGGREGATABLE); }
#line 3314 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 78: /* attribute: tANNOTATION '(' aSTRING ')'  */
#line 519 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_ANNOTATION, (yyvsp[-1].str)); }
#line 3320 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 79: /* attribute: tAPPOBJECT  */
#line 520 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_APPOBJECT); }
#line 3326 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 80: /* attribute: tASYNC  */
#line 521 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_ASYNC); }
#line 3332 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 81: /* attribute: tAUTOHANDLE  */
#line 522 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_AUTO_HANDLE); }
#line 3338 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 82: /* attribute: tBINDABLE  */
#line 523 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_BINDABLE); }
#line 3344 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 83: /* attribute: tBROADCAST  */
#line 524 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_BROADCAST); }
#line 3350 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 84: /* attribute: tCALLAS '(' ident ')'  */
#line 525 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_CALLAS, (yyvsp[-1].var)); }
#line 3356 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 85: /* attribute: tCASE '(' expr_list_int_const ')'  */
#line 526 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_CASE, (yyvsp[-1].expr_list)); }
#line 3362 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 86: /* attribute: tCODE  */
#line 527 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_CODE); }
#line 3368 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 87: /* attribute: tCOMMSTATUS  */
#line 528 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_COMMSTATUS); }
#line 3374 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 88: /* attribute: tCONTEXTHANDLE  */
#line 529 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_CONTEXTHANDLE, 0); }
#line 3380 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 89: /* attribute: tCONTEXTHANDLENOSERIALIZE  */
#line 530 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_CONTEXTHANDLE, 0); /* RPC_CONTEXT_HANDLE_DONT_SERIALIZE */ }
#line 3386 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 90: /* attribute: tCONTEXTHANDLESERIALIZE  */
#line 531 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_CONTEXTHANDLE, 0); /* RPC_CONTEXT_HANDLE_SERIALIZE */ }
#line 3392 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 91: /* attribute: tCONTROL  */
#line 532 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_CONTROL); }
#line 3398 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 92: /* attribute: tDECODE  */
#line 533 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DECODE); }
#line 3404 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 93: /* attribute: tDEFAULT  */
#line 534 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DEFAULT); }
#line 3410 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 94: /* attribute: tDEFAULTBIND  */
#line 535 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DEFAULTBIND); }
#line 3416 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 95: /* attribute: tDEFAULTCOLLELEM  */
#line 536 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DEFAULTCOLLELEM); }
#line 3422 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 96: /* attribute: tDEFAULTVALUE '(' expr_const ')'  */
#line 537 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_DEFAULTVALUE, (yyvsp[-1].expr)); }
#line 3428 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 97: /* attribute: tDEFAULTVTABLE  */
#line 538 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DEFAULTVTABLE); }
#line 3434 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 98: /* attribute: tDISABLECONSISTENCYCHECK  */
#line 539 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DISABLECONSISTENCYCHECK); }
#line 3440 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 99: /* attribute: tDISPLAYBIND  */
#line 540 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DISPLAYBIND); }
#line 3446 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 100: /* attribute: tDLLNAME '(' aSTRING ')'  */
#line 541 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_DLLNAME, (yyvsp[-1].str)); }
#line 3452 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 101: /* attribute: tDUAL  */
#line 542 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DUAL); }
#line 3458 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 102: /* attribute: tENABLEALLOCATE  */
#line 543 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_ENABLEALLOCATE); }
#line 3464 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 103: /* attribute: tENCODE  */
#line 544 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_ENCODE); }
#line 3470 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 104: /* attribute: tENDPOINT '(' str_list ')'  */
#line 545 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_ENDPOINT, (yyvsp[-1].str_list)); }
#line 3476 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 105: /* attribute: tENTRY '(' expr_const ')'  */
#line 546 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_ENTRY, (yyvsp[-1].expr)); }
#line 3482 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 106: /* attribute: tEXPLICITHANDLE  */
#line 547 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_EXPLICIT_HANDLE); }
#line 3488 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 107: /* attribute: tFAULTSTATUS  */
#line 548 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_FAULTSTATUS); }
#line 3494 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 108: /* attribute: tFORCEALLOCATE  */
#line 549 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_FORCEALLOCATE); }
#line 3500 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 109: /* attribute: tHANDLE  */
#line 550 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_HANDLE); }
#line 3506 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 110: /* attribute: tHELPCONTEXT '(' expr_int_const ')'  */
#line 551 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_HELPCONTEXT, (yyvsp[-1].expr)); }
#line 3512 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 111: /* attribute: tHELPFILE '(' aSTRING ')'  */
#line 552 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_HELPFILE, (yyvsp[-1].str)); }
#line 3518 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 112: /* attribute: tHELPSTRING '(' aSTRING ')'  */
#line 553 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_HELPSTRING, (yyvsp[-1].str)); }
#line 3524 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 113: /* attribute: tHELPSTRINGCONTEXT '(' expr_int_const ')'  */
#line 554 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                        { (yyval.attr) = make_attrp(ATTR_HELPSTRINGCONTEXT, (yyvsp[-1].expr)); }
#line 3530 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 114: /* attribute: tHELPSTRINGDLL '(' aSTRING ')'  */
#line 555 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_HELPSTRINGDLL, (yyvsp[-1].str)); }
#line 3536 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 115: /* attribute: tHIDDEN  */
#line 556 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_HIDDEN); }
#line 3542 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 116: /* attribute: tID '(' expr_int_const ')'  */
#line 557 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_ID, (yyvsp[-1].expr)); }
#line 3548 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 117: /* attribute: tIDEMPOTENT  */
#line 558 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_IDEMPOTENT); }
#line 3554 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 118: /* attribute: tIGNORE  */
#line 559 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_IGNORE); }
#line 3560 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 119: /* attribute: tIIDIS '(' expr ')'  */
#line 560 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_IIDIS, (yyvsp[-1].expr)); }
#line 3566 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 120: /* attribute: tIMMEDIATEBIND  */
#line 561 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_IMMEDIATEBIND); }
#line 3572 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 121: /* attribute: tIMPLICITHANDLE '(' arg ')'  */
#line 562 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_IMPLICIT_HANDLE, (yyvsp[-1].var)); }
#line 3578 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 122: /* attribute: tIN  */
#line 563 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_IN); }
#line 3584 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 123: /* attribute: tINPUTSYNC  */
#line 564 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_INPUTSYNC); }
#line 3590 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 124: /* attribute: tLENGTHIS '(' m_exprs ')'  */
#line 565 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_LENGTHIS, (yyvsp[-1].expr_list)); }
#line 3596 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 125: /* attribute: tLCID '(' expr_int_const ')'  */
#line 566 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_LIBLCID, (yyvsp[-1].expr)); }
#line 3602 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 126: /* attribute: tLCID  */
#line 567 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PARAMLCID); }
#line 3608 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 127: /* attribute: tLICENSED  */
#line 568 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_LICENSED); }
#line 3614 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 128: /* attribute: tLOCAL  */
#line 569 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_LOCAL); }
#line 3620 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 129: /* attribute: tMAYBE  */
#line 570 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_MAYBE); }
#line 3626 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 130: /* attribute: tMESSAGE  */
#line 571 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_MESSAGE); }
#line 3632 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 131: /* attribute: tNOCODE  */
#line 572 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_NOCODE); }
#line 3638 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 132: /* attribute: tNONBROWSABLE  */
#line 573 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_NONBROWSABLE); }
#line 3644 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 133: /* attribute: tNONCREATABLE  */
#line 574 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_NONCREATABLE); }
#line 3650 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 134: /* attribute: tNONEXTENSIBLE  */
#line 575 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_NONEXTENSIBLE); }
#line 3656 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 135: /* attribute: tNOTIFY  */
#line 576 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_NOTIFY); }
#line 3662 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 136: /* attribute: tNOTIFYFLAG  */
#line 577 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_NOTIFYFLAG); }
#line 3668 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 137: /* attribute: tOBJECT  */
#line 578 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_OBJECT); }
#line 3674 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 138: /* attribute: tODL  */
#line 579 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_ODL); }
#line 3680 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 139: /* attribute: tOLEAUTOMATION  */
#line 580 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_OLEAUTOMATION); }
#line 3686 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 140: /* attribute: tOPTIMIZE '(' aSTRING ')'  */
#line 581 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_OPTIMIZE, (yyvsp[-1].str)); }
#line 3692 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 141: /* attribute: tOPTIONAL  */
#line 582 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_OPTIONAL); }
#line 3698 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 142: /* attribute: tOUT  */
#line 583 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_OUT); }
#line 3704 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 143: /* attribute: tPARTIALIGNORE  */
#line 584 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PARTIALIGNORE); }
#line 3710 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 144: /* attribute: tPOINTERDEFAULT '(' pointer_type ')'  */
#line 585 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_POINTERDEFAULT, (yyvsp[-1].num)); }
#line 3716 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 145: /* attribute: tPROGID '(' aSTRING ')'  */
#line 586 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_PROGID, (yyvsp[-1].str)); }
#line 3722 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 146: /* attribute: tPROPGET  */
#line 587 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PROPGET); }
#line 3728 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 147: /* attribute: tPROPPUT  */
#line 588 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PROPPUT); }
#line 3734 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 148: /* attribute: tPROPPUTREF  */
#line 589 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PROPPUTREF); }
#line 3740 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 149: /* attribute: tPROXY  */
#line 590 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PROXY); }
#line 3746 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 150: /* attribute: tPUBLIC  */
#line 591 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_PUBLIC); }
#line 3752 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 151: /* attribute: tRANGE '(' expr_int_const ',' expr_int_const ')'  */
#line 593 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { expr_list_t *list = append_expr( NULL, (yyvsp[-3].expr) );
						  list = append_expr( list, (yyvsp[-1].expr) );
						  (yyval.attr) = make_attrp(ATTR_RANGE, list); }
#line 3760 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 152: /* attribute: tREADONLY  */
#line 596 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_READONLY); }
#line 3766 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 153: /* attribute: tREPRESENTAS '(' type ')'  */
#line 597 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_REPRESENTAS, (yyvsp[-1].type)); }
#line 3772 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 154: /* attribute: tREQUESTEDIT  */
#line 598 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_REQUESTEDIT); }
#line 3778 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 155: /* attribute: tRESTRICTED  */
#line 599 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_RESTRICTED); }
#line 3784 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 156: /* attribute: tRETVAL  */
#line 600 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_RETVAL); }
#line 3790 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 157: /* attribute: tSIZEIS '(' m_exprs ')'  */
#line 601 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_SIZEIS, (yyvsp[-1].expr_list)); }
#line 3796 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 158: /* attribute: tSOURCE  */
#line 602 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_SOURCE); }
#line 3802 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 159: /* attribute: tSTRICTCONTEXTHANDLE  */
#line 603 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_STRICTCONTEXTHANDLE); }
#line 3808 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 160: /* attribute: tSTRING  */
#line 604 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_STRING); }
#line 3814 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 161: /* attribute: tSWITCHIS '(' expr ')'  */
#line 605 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_SWITCHIS, (yyvsp[-1].expr)); }
#line 3820 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 162: /* attribute: tSWITCHTYPE '(' type ')'  */
#line 606 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_SWITCHTYPE, (yyvsp[-1].type)); }
#line 3826 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 163: /* attribute: tTRANSMITAS '(' type ')'  */
#line 607 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_TRANSMITAS, (yyvsp[-1].type)); }
#line 3832 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 164: /* attribute: tTHREADING '(' threading_type ')'  */
#line 608 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_THREADING, (yyvsp[-1].num)); }
#line 3838 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 165: /* attribute: tUIDEFAULT  */
#line 609 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_UIDEFAULT); }
#line 3844 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 166: /* attribute: tUSESGETLASTERROR  */
#line 610 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_USESGETLASTERROR); }
#line 3850 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 167: /* attribute: tUSERMARSHAL '(' type ')'  */
#line 611 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_USERMARSHAL, (yyvsp[-1].type)); }
#line 3856 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 168: /* attribute: tUUID '(' uuid_string ')'  */
#line 612 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_UUID, (yyvsp[-1].uuid)); }
#line 3862 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 169: /* attribute: tASYNCUUID '(' uuid_string ')'  */
#line 613 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_ASYNCUUID, (yyvsp[-1].uuid)); }
#line 3868 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 170: /* attribute: tV1ENUM  */
#line 614 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_V1ENUM); }
#line 3874 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 171: /* attribute: tVARARG  */
#line 615 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_VARARG); }
#line 3880 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 172: /* attribute: tVERSION '(' version ')'  */
#line 616 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_VERSION, (yyvsp[-1].num)); }
#line 3886 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 173: /* attribute: tVIPROGID '(' aSTRING ')'  */
#line 617 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_VIPROGID, (yyvsp[-1].str)); }
#line 3892 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 174: /* attribute: tWIREMARSHAL '(' type ')'  */
#line 618 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrp(ATTR_WIREMARSHAL, (yyvsp[-1].type)); }
#line 3898 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 175: /* attribute: pointer_type  */
#line 619 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_POINTERTYPE, (yyvsp[0].num)); }
#line 3904 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 177: /* uuid_string: aSTRING  */
#line 624 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { if (!is_valid_uuid((yyvsp[0].str)))
						    error_loc("invalid UUID: %s\n", (yyvsp[0].str));
						  (yyval.uuid) = parse_uuid((yyvsp[0].str)); }
#line 3912 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 178: /* callconv: tCDECL  */
#line 629 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = xstrdup("__cdecl"); }
#line 3918 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 179: /* callconv: tFASTCALL  */
#line 630 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = xstrdup("__fastcall"); }
#line 3924 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 180: /* callconv: tPASCAL  */
#line 631 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = xstrdup("__pascal"); }
#line 3930 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 181: /* callconv: tSTDCALL  */
#line 632 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = xstrdup("__stdcall"); }
#line 3936 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 182: /* cases: %empty  */
#line 635 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 3942 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 183: /* cases: cases case  */
#line 636 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = append_var( (yyvsp[-1].var_list), (yyvsp[0].var) ); }
#line 3948 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 184: /* case: tCASE expr_int_const ':' union_field  */
#line 639 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { attr_t *a = make_attrp(ATTR_CASE, append_expr( NULL, (yyvsp[-2].expr) ));
						  (yyval.var) = (yyvsp[0].var); if (!(yyval.var)) (yyval.var) = make_var(NULL);
						  (yyval.var)->attrs = append_attr( (yyval.var)->attrs, a );
						}
#line 3957 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 185: /* case: tDEFAULT ':' union_field  */
#line 643 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { attr_t *a = make_attr(ATTR_DEFAULT);
						  (yyval.var) = (yyvsp[0].var); if (!(yyval.var)) (yyval.var) = make_var(NULL);
						  (yyval.var)->attrs = append_attr( (yyval.var)->attrs, a );
						}
#line 3966 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 186: /* enums: %empty  */
#line 649 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 3972 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 187: /* enums: enum_list ','  */
#line 650 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = (yyvsp[-1].var_list); }
#line 3978 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 189: /* enum_list: enum  */
#line 654 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { if (!(yyvsp[0].var)->eval)
						    (yyvsp[0].var)->eval = make_exprl(EXPR_NUM, 0 /* default for first enum entry */);
                                                  (yyval.var_list) = append_var( NULL, (yyvsp[0].var) );
						}
#line 3987 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 190: /* enum_list: enum_list ',' enum  */
#line 658 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { if (!(yyvsp[0].var)->eval)
                                                  {
                                                    var_t *last = LIST_ENTRY( list_tail((yyval.var_list)), var_t, entry );
                                                    enum expr_type type = EXPR_NUM;
                                                    if (last->eval->type == EXPR_HEXNUM) type = EXPR_HEXNUM;
                                                    if (last->eval->cval + 1 < 0) type = EXPR_HEXNUM;
                                                    (yyvsp[0].var)->eval = make_exprl(type, last->eval->cval + 1);
                                                  }
                                                  (yyval.var_list) = append_var( (yyvsp[-2].var_list), (yyvsp[0].var) );
						}
#line 4002 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 191: /* enum: ident '=' expr_int_const  */
#line 670 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = reg_const((yyvsp[-2].var));
						  (yyval.var)->eval = (yyvsp[0].expr);
                                                  (yyval.var)->type = type_new_int(TYPE_BASIC_INT, 0);
						}
#line 4011 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 192: /* enum: ident  */
#line 674 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = reg_const((yyvsp[0].var));
                                                  (yyval.var)->type = type_new_int(TYPE_BASIC_INT, 0);
						}
#line 4019 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 193: /* enumdef: tENUM t_ident '{' enums '}'  */
#line 679 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_enum((yyvsp[-3].str), current_namespace, TRUE, (yyvsp[-1].var_list)); }
#line 4025 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 194: /* m_exprs: m_expr  */
#line 682 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr_list) = append_expr( NULL, (yyvsp[0].expr) ); }
#line 4031 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 195: /* m_exprs: m_exprs ',' m_expr  */
#line 683 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr_list) = append_expr( (yyvsp[-2].expr_list), (yyvsp[0].expr) ); }
#line 4037 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 196: /* m_expr: %empty  */
#line 686 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr(EXPR_VOID); }
#line 4043 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 198: /* expr: aNUM  */
#line 690 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprl(EXPR_NUM, (yyvsp[0].num)); }
#line 4049 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 199: /* expr: aHEXNUM  */
#line 691 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprl(EXPR_HEXNUM, (yyvsp[0].num)); }
#line 4055 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 200: /* expr: aDOUBLE  */
#line 692 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprd(EXPR_DOUBLE, (yyvsp[0].dbl)); }
#line 4061 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 201: /* expr: tFALSE  */
#line 693 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprl(EXPR_TRUEFALSE, 0); }
#line 4067 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 202: /* expr: tNULL  */
#line 694 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprl(EXPR_NUM, 0); }
#line 4073 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 203: /* expr: tTRUE  */
#line 695 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprl(EXPR_TRUEFALSE, 1); }
#line 4079 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 204: /* expr: aSTRING  */
#line 696 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprs(EXPR_STRLIT, (yyvsp[0].str)); }
#line 4085 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 205: /* expr: aWSTRING  */
#line 697 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprs(EXPR_WSTRLIT, (yyvsp[0].str)); }
#line 4091 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 206: /* expr: aSQSTRING  */
#line 698 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprs(EXPR_CHARCONST, (yyvsp[0].str)); }
#line 4097 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 207: /* expr: aIDENTIFIER  */
#line 699 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprs(EXPR_IDENTIFIER, (yyvsp[0].str)); }
#line 4103 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 208: /* expr: expr '?' expr ':' expr  */
#line 700 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr3(EXPR_COND, (yyvsp[-4].expr), (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4109 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 209: /* expr: expr LOGICALOR expr  */
#line 701 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_LOGOR, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4115 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 210: /* expr: expr LOGICALAND expr  */
#line 702 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_LOGAND, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4121 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 211: /* expr: expr '|' expr  */
#line 703 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_OR , (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4127 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 212: /* expr: expr '^' expr  */
#line 704 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_XOR, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4133 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 213: /* expr: expr '&' expr  */
#line 705 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_AND, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4139 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 214: /* expr: expr EQUALITY expr  */
#line 706 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_EQUALITY, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4145 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 215: /* expr: expr INEQUALITY expr  */
#line 707 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_INEQUALITY, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4151 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 216: /* expr: expr '>' expr  */
#line 708 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_GTR, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4157 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 217: /* expr: expr '<' expr  */
#line 709 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_LESS, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4163 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 218: /* expr: expr GREATEREQUAL expr  */
#line 710 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_GTREQL, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4169 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 219: /* expr: expr LESSEQUAL expr  */
#line 711 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_LESSEQL, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4175 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 220: /* expr: expr SHL expr  */
#line 712 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_SHL, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4181 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 221: /* expr: expr SHR expr  */
#line 713 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_SHR, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4187 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 222: /* expr: expr '+' expr  */
#line 714 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_ADD, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4193 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 223: /* expr: expr '-' expr  */
#line 715 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_SUB, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4199 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 224: /* expr: expr '%' expr  */
#line 716 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_MOD, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4205 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 225: /* expr: expr '*' expr  */
#line 717 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_MUL, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4211 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 226: /* expr: expr '/' expr  */
#line 718 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_DIV, (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 4217 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 227: /* expr: '!' expr  */
#line 719 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr1(EXPR_LOGNOT, (yyvsp[0].expr)); }
#line 4223 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 228: /* expr: '~' expr  */
#line 720 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr1(EXPR_NOT, (yyvsp[0].expr)); }
#line 4229 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 229: /* expr: '+' expr  */
#line 721 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr1(EXPR_POS, (yyvsp[0].expr)); }
#line 4235 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 230: /* expr: '-' expr  */
#line 722 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr1(EXPR_NEG, (yyvsp[0].expr)); }
#line 4241 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 231: /* expr: '&' expr  */
#line 723 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr1(EXPR_ADDRESSOF, (yyvsp[0].expr)); }
#line 4247 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 232: /* expr: '*' expr  */
#line 724 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr1(EXPR_PPTR, (yyvsp[0].expr)); }
#line 4253 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 233: /* expr: expr MEMBERPTR aIDENTIFIER  */
#line 725 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_MEMBER, make_expr1(EXPR_PPTR, (yyvsp[-2].expr)), make_exprs(EXPR_IDENTIFIER, (yyvsp[0].str))); }
#line 4259 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 234: /* expr: expr '.' aIDENTIFIER  */
#line 726 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_MEMBER, (yyvsp[-2].expr), make_exprs(EXPR_IDENTIFIER, (yyvsp[0].str))); }
#line 4265 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 235: /* expr: '(' decl_spec m_abstract_declarator ')' expr  */
#line 728 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprt(EXPR_CAST, declare_var(NULL, (yyvsp[-3].declspec), (yyvsp[-2].declarator), 0), (yyvsp[0].expr)); free((yyvsp[-3].declspec)); free((yyvsp[-2].declarator)); }
#line 4271 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 236: /* expr: tSIZEOF '(' decl_spec m_abstract_declarator ')'  */
#line 730 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_exprt(EXPR_SIZEOF, declare_var(NULL, (yyvsp[-2].declspec), (yyvsp[-1].declarator), 0), NULL); free((yyvsp[-2].declspec)); free((yyvsp[-1].declarator)); }
#line 4277 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 237: /* expr: expr '[' expr ']'  */
#line 731 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = make_expr2(EXPR_ARRAY, (yyvsp[-3].expr), (yyvsp[-1].expr)); }
#line 4283 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 238: /* expr: '(' expr ')'  */
#line 732 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = (yyvsp[-1].expr); }
#line 4289 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 239: /* expr_list_int_const: expr_int_const  */
#line 735 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr_list) = append_expr( NULL, (yyvsp[0].expr) ); }
#line 4295 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 240: /* expr_list_int_const: expr_list_int_const ',' expr_int_const  */
#line 736 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                        { (yyval.expr_list) = append_expr( (yyvsp[-2].expr_list), (yyvsp[0].expr) ); }
#line 4301 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 241: /* expr_int_const: expr  */
#line 739 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = (yyvsp[0].expr);
						  if (!(yyval.expr)->is_const)
						      error_loc("expression is not an integer constant\n");
						}
#line 4310 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 242: /* expr_const: expr  */
#line 745 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = (yyvsp[0].expr);
						  if (!(yyval.expr)->is_const && (yyval.expr)->type != EXPR_STRLIT && (yyval.expr)->type != EXPR_WSTRLIT)
						      error_loc("expression is not constant\n");
						}
#line 4319 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 243: /* fields: %empty  */
#line 751 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 4325 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 244: /* fields: fields field  */
#line 752 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = append_var_list((yyvsp[-1].var_list), (yyvsp[0].var_list)); }
#line 4331 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 245: /* field: m_attributes decl_spec struct_declarator_list ';'  */
#line 756 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { const char *first = LIST_ENTRY(list_head((yyvsp[-1].declarator_list)), declarator_t, entry)->var->name;
						  check_field_attrs(first, (yyvsp[-3].attr_list));
						  (yyval.var_list) = set_var_types((yyvsp[-3].attr_list), (yyvsp[-2].declspec), (yyvsp[-1].declarator_list));
						}
#line 4340 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 246: /* field: m_attributes uniondef ';'  */
#line 760 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { var_t *v = make_var(NULL);
						  v->type = (yyvsp[-1].type); v->attrs = (yyvsp[-2].attr_list);
						  (yyval.var_list) = append_var(NULL, v);
						}
#line 4349 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 247: /* ne_union_field: s_field ';'  */
#line 767 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = (yyvsp[-1].var); }
#line 4355 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 248: /* ne_union_field: attributes ';'  */
#line 768 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = make_var(NULL); (yyval.var)->attrs = (yyvsp[-1].attr_list); }
#line 4361 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 249: /* ne_union_fields: %empty  */
#line 771 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 4367 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 250: /* ne_union_fields: ne_union_fields ne_union_field  */
#line 772 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = append_var( (yyvsp[-1].var_list), (yyvsp[0].var) ); }
#line 4373 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 251: /* union_field: s_field ';'  */
#line 776 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = (yyvsp[-1].var); }
#line 4379 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 252: /* union_field: ';'  */
#line 777 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = NULL; }
#line 4385 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 253: /* s_field: m_attributes decl_spec declarator  */
#line 780 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = declare_var(check_field_attrs((yyvsp[0].declarator)->var->name, (yyvsp[-2].attr_list)),
						                (yyvsp[-1].declspec), (yyvsp[0].declarator), FALSE);
						  free((yyvsp[0].declarator));
						}
#line 4394 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 254: /* s_field: m_attributes structdef  */
#line 784 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { var_t *v = make_var(NULL);
						  v->type = (yyvsp[0].type); v->attrs = (yyvsp[-1].attr_list);
						  (yyval.var) = v;
						}
#line 4403 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 255: /* funcdef: declaration  */
#line 790 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = (yyvsp[0].var);
						  if (type_get_type((yyval.var)->type) != TYPE_FUNCTION)
						    error_loc("only methods may be declared inside the methods section of a dispinterface\n");
						  check_function_attrs((yyval.var)->name, (yyval.var)->attrs);
						}
#line 4413 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 256: /* declaration: attributes decl_spec init_declarator  */
#line 799 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = declare_var((yyvsp[-2].attr_list), (yyvsp[-1].declspec), (yyvsp[0].declarator), FALSE);
						  free((yyvsp[0].declarator));
						}
#line 4421 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 257: /* declaration: decl_spec init_declarator  */
#line 802 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = declare_var(NULL, (yyvsp[-1].declspec), (yyvsp[0].declarator), FALSE);
						  free((yyvsp[0].declarator));
						}
#line 4429 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 258: /* m_ident: %empty  */
#line 807 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = NULL; }
#line 4435 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 260: /* t_ident: %empty  */
#line 811 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = NULL; }
#line 4441 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 261: /* t_ident: aIDENTIFIER  */
#line 812 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[0].str); }
#line 4447 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 262: /* t_ident: aKNOWNTYPE  */
#line 813 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[0].str); }
#line 4453 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 263: /* ident: aIDENTIFIER  */
#line 816 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = make_var((yyvsp[0].str)); }
#line 4459 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 264: /* ident: aKNOWNTYPE  */
#line 818 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var) = make_var((yyvsp[0].str)); }
#line 4465 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 265: /* base_type: tBYTE  */
#line 821 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4471 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 266: /* base_type: tWCHAR  */
#line 822 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4477 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 268: /* base_type: tSIGNED int_std  */
#line 824 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(type_basic_get_type((yyvsp[0].type)), -1); }
#line 4483 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 269: /* base_type: tUNSIGNED int_std  */
#line 825 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(type_basic_get_type((yyvsp[0].type)), 1); }
#line 4489 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 270: /* base_type: tUNSIGNED  */
#line 826 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT, 1); }
#line 4495 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 271: /* base_type: tFLOAT  */
#line 827 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4501 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 272: /* base_type: tDOUBLE  */
#line 828 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4507 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 273: /* base_type: tBOOLEAN  */
#line 829 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4513 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 274: /* base_type: tERRORSTATUST  */
#line 830 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4519 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 275: /* base_type: tHANDLET  */
#line 831 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 4525 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 278: /* int_std: tINT  */
#line 838 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT, 0); }
#line 4531 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 279: /* int_std: tSHORT m_int  */
#line 839 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT16, 0); }
#line 4537 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 280: /* int_std: tSMALL  */
#line 840 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT8, 0); }
#line 4543 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 281: /* int_std: tLONG m_int  */
#line 841 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_LONG, 0); }
#line 4549 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 282: /* int_std: tHYPER m_int  */
#line 842 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_HYPER, 0); }
#line 4555 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 283: /* int_std: tINT64  */
#line 843 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT64, 0); }
#line 4561 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 284: /* int_std: tCHAR  */
#line 844 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_CHAR, 0); }
#line 4567 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 285: /* int_std: tINT32  */
#line 845 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT32, 0); }
#line 4573 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 286: /* int_std: tINT3264  */
#line 846 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_int(TYPE_BASIC_INT3264, 0); }
#line 4579 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 287: /* coclass: tCOCLASS aIDENTIFIER  */
#line 849 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_coclass((yyvsp[0].str)); }
#line 4585 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 288: /* coclass: tCOCLASS aKNOWNTYPE  */
#line 850 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type((yyvsp[0].str), NULL, 0);
						  if (type_get_type_detect_alias((yyval.type)) != TYPE_COCLASS)
						    error_loc("%s was not declared a coclass at %s:%d\n",
							      (yyvsp[0].str), (yyval.type)->loc_info.input_name,
							      (yyval.type)->loc_info.line_number);
						}
#line 4596 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 289: /* coclasshdr: attributes coclass  */
#line 858 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type);
						  check_def((yyval.type));
						  (yyval.type)->attrs = check_coclass_attrs((yyvsp[0].type)->name, (yyvsp[-1].attr_list));
						}
#line 4605 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 290: /* coclassdef: coclasshdr '{' coclass_ints '}' semicolon_opt  */
#line 865 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_coclass_define((yyvsp[-4].type), (yyvsp[-2].ifref_list)); }
#line 4611 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 291: /* namespacedef: tNAMESPACE aIDENTIFIER  */
#line 868 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.str) = (yyvsp[0].str); }
#line 4617 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 292: /* coclass_ints: %empty  */
#line 871 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.ifref_list) = NULL; }
#line 4623 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 293: /* coclass_ints: coclass_ints coclass_int  */
#line 872 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.ifref_list) = append_ifref( (yyvsp[-1].ifref_list), (yyvsp[0].ifref) ); }
#line 4629 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 294: /* coclass_int: m_attributes interfacedec  */
#line 876 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.ifref) = make_ifref((yyvsp[0].type)); (yyval.ifref)->attrs = (yyvsp[-1].attr_list); }
#line 4635 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 295: /* dispinterface: tDISPINTERFACE aIDENTIFIER  */
#line 879 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = get_type(TYPE_INTERFACE, (yyvsp[0].str), current_namespace, 0); }
#line 4641 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 296: /* dispinterface: tDISPINTERFACE aKNOWNTYPE  */
#line 880 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = get_type(TYPE_INTERFACE, (yyvsp[0].str), current_namespace, 0); }
#line 4647 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 297: /* dispinterfacehdr: attributes dispinterface  */
#line 883 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { attr_t *attrs;
						  (yyval.type) = (yyvsp[0].type);
						  check_def((yyval.type));
						  attrs = make_attr(ATTR_DISPINTERFACE);
						  (yyval.type)->attrs = append_attr( check_dispiface_attrs((yyvsp[0].type)->name, (yyvsp[-1].attr_list)), attrs );
						  (yyval.type)->defined = TRUE;
						}
#line 4659 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 298: /* dispint_props: tPROPERTIES ':'  */
#line 892 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 4665 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 299: /* dispint_props: dispint_props s_field ';'  */
#line 893 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = append_var( (yyvsp[-2].var_list), (yyvsp[-1].var) ); }
#line 4671 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 300: /* dispint_meths: tMETHODS ':'  */
#line 896 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = NULL; }
#line 4677 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 301: /* dispint_meths: dispint_meths funcdef ';'  */
#line 897 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.var_list) = append_var( (yyvsp[-2].var_list), (yyvsp[-1].var) ); }
#line 4683 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 302: /* dispinterfacedef: dispinterfacehdr '{' dispint_props dispint_meths '}'  */
#line 903 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-4].type);
						  type_dispinterface_define((yyval.type), (yyvsp[-2].var_list), (yyvsp[-1].var_list));
						}
#line 4691 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 303: /* dispinterfacedef: dispinterfacehdr '{' interface ';' '}'  */
#line 907 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-4].type);
						  type_dispinterface_define_from_iface((yyval.type), (yyvsp[-2].type));
						}
#line 4699 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 304: /* inherit: %empty  */
#line 912 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = NULL; }
#line 4705 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 305: /* inherit: ':' aKNOWNTYPE  */
#line 913 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error2((yyvsp[0].str), 0); }
#line 4711 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 306: /* interface: tINTERFACE aIDENTIFIER  */
#line 916 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = get_type(TYPE_INTERFACE, (yyvsp[0].str), current_namespace, 0); }
#line 4717 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 307: /* interface: tINTERFACE aKNOWNTYPE  */
#line 917 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = get_type(TYPE_INTERFACE, (yyvsp[0].str), current_namespace, 0); }
#line 4723 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 308: /* interfacehdr: attributes interface  */
#line 920 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.ifinfo).interface = (yyvsp[0].type);
						  (yyval.ifinfo).old_pointer_default = pointer_default;
						  if (is_attr((yyvsp[-1].attr_list), ATTR_POINTERDEFAULT))
						    pointer_default = get_attrv((yyvsp[-1].attr_list), ATTR_POINTERDEFAULT);
						  check_def((yyvsp[0].type));
						  (yyvsp[0].type)->attrs = check_iface_attrs((yyvsp[0].type)->name, (yyvsp[-1].attr_list));
						  (yyvsp[0].type)->defined = TRUE;
						}
#line 4736 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 309: /* interfacedef: interfacehdr inherit '{' int_statements '}' semicolon_opt  */
#line 931 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-5].ifinfo).interface;
						  if((yyval.type) == (yyvsp[-4].type))
						    error_loc("Interface can't inherit from itself\n");
						  type_interface_define((yyval.type), (yyvsp[-4].type), (yyvsp[-2].stmt_list));
						  check_async_uuid((yyval.type));
						  pointer_default = (yyvsp[-5].ifinfo).old_pointer_default;
						}
#line 4748 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 310: /* interfacedef: interfacehdr ':' aIDENTIFIER '{' import int_statements '}' semicolon_opt  */
#line 942 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-7].ifinfo).interface;
						  type_interface_define((yyval.type), find_type_or_error2((yyvsp[-5].str), 0), (yyvsp[-2].stmt_list));
						  pointer_default = (yyvsp[-7].ifinfo).old_pointer_default;
						}
#line 4757 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 311: /* interfacedef: dispinterfacedef semicolon_opt  */
#line 946 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-1].type); }
#line 4763 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 312: /* interfacedec: interface ';'  */
#line 950 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-1].type); }
#line 4769 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 313: /* interfacedec: dispinterface ';'  */
#line 951 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-1].type); }
#line 4775 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 314: /* module: tMODULE aIDENTIFIER  */
#line 954 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_module((yyvsp[0].str)); }
#line 4781 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 315: /* module: tMODULE aKNOWNTYPE  */
#line 955 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_module((yyvsp[0].str)); }
#line 4787 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 316: /* modulehdr: attributes module  */
#line 958 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type);
						  (yyval.type)->attrs = check_module_attrs((yyvsp[0].type)->name, (yyvsp[-1].attr_list));
						}
#line 4795 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 317: /* moduledef: modulehdr '{' int_statements '}' semicolon_opt  */
#line 964 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[-4].type);
                                                  type_module_define((yyval.type), (yyvsp[-2].stmt_list));
						}
#line 4803 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 318: /* storage_cls_spec: tEXTERN  */
#line 970 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stgclass) = STG_EXTERN; }
#line 4809 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 319: /* storage_cls_spec: tSTATIC  */
#line 971 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stgclass) = STG_STATIC; }
#line 4815 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 320: /* storage_cls_spec: tREGISTER  */
#line 972 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.stgclass) = STG_REGISTER; }
#line 4821 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 321: /* function_specifier: tINLINE  */
#line 976 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_INLINE); }
#line 4827 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 322: /* type_qualifier: tCONST  */
#line 980 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_CONST); }
#line 4833 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 323: /* m_type_qual_list: %empty  */
#line 983 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = NULL; }
#line 4839 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 324: /* m_type_qual_list: m_type_qual_list type_qualifier  */
#line 984 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = append_attr((yyvsp[-1].attr_list), (yyvsp[0].attr)); }
#line 4845 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 325: /* decl_spec: type m_decl_spec_no_type  */
#line 987 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declspec) = make_decl_spec((yyvsp[-1].type), (yyvsp[0].declspec), NULL, NULL, STG_NONE); }
#line 4851 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 326: /* decl_spec: decl_spec_no_type type m_decl_spec_no_type  */
#line 989 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declspec) = make_decl_spec((yyvsp[-1].type), (yyvsp[-2].declspec), (yyvsp[0].declspec), NULL, STG_NONE); }
#line 4857 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 327: /* m_decl_spec_no_type: %empty  */
#line 992 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declspec) = NULL; }
#line 4863 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 329: /* decl_spec_no_type: type_qualifier m_decl_spec_no_type  */
#line 997 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declspec) = make_decl_spec(NULL, (yyvsp[0].declspec), NULL, (yyvsp[-1].attr), STG_NONE); }
#line 4869 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 330: /* decl_spec_no_type: function_specifier m_decl_spec_no_type  */
#line 998 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                  { (yyval.declspec) = make_decl_spec(NULL, (yyvsp[0].declspec), NULL, (yyvsp[-1].attr), STG_NONE); }
#line 4875 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 331: /* decl_spec_no_type: storage_cls_spec m_decl_spec_no_type  */
#line 999 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declspec) = make_decl_spec(NULL, (yyvsp[0].declspec), NULL, NULL, (yyvsp[-1].stgclass)); }
#line 4881 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 332: /* declarator: '*' m_type_qual_list declarator  */
#line 1004 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type = append_chain_type((yyval.declarator)->type, type_new_pointer(pointer_default, NULL, (yyvsp[-1].attr_list))); }
#line 4887 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 333: /* declarator: callconv declarator  */
#line 1005 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); if ((yyval.declarator)->func_type) (yyval.declarator)->func_type->attrs = append_attr((yyval.declarator)->func_type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str)));
						           else if ((yyval.declarator)->type) (yyval.declarator)->type->attrs = append_attr((yyval.declarator)->type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str))); }
#line 4894 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 335: /* direct_declarator: ident  */
#line 1011 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator((yyvsp[0].var)); }
#line 4900 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 336: /* direct_declarator: '(' declarator ')'  */
#line 1012 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); }
#line 4906 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 337: /* direct_declarator: direct_declarator array  */
#line 1013 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); (yyval.declarator)->type = append_array((yyval.declarator)->type, (yyvsp[0].expr)); }
#line 4912 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 338: /* direct_declarator: direct_declarator '(' m_args ')'  */
#line 1014 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-3].declarator);
						  (yyval.declarator)->func_type = append_chain_type((yyval.declarator)->type, type_new_function((yyvsp[-1].var_list)));
						  (yyval.declarator)->type = NULL;
						}
#line 4921 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 339: /* abstract_declarator: '*' m_type_qual_list m_abstract_declarator  */
#line 1023 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type = append_chain_type((yyval.declarator)->type, type_new_pointer(pointer_default, NULL, (yyvsp[-1].attr_list))); }
#line 4927 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 340: /* abstract_declarator: callconv m_abstract_declarator  */
#line 1024 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); if ((yyval.declarator)->func_type) (yyval.declarator)->func_type->attrs = append_attr((yyval.declarator)->func_type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str)));
						           else if ((yyval.declarator)->type) (yyval.declarator)->type->attrs = append_attr((yyval.declarator)->type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str))); }
#line 4934 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 342: /* abstract_declarator_no_direct: '*' m_type_qual_list m_any_declarator  */
#line 1032 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type = append_chain_type((yyval.declarator)->type, type_new_pointer(pointer_default, NULL, (yyvsp[-1].attr_list))); }
#line 4940 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 343: /* abstract_declarator_no_direct: callconv m_any_declarator  */
#line 1033 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); if ((yyval.declarator)->func_type) (yyval.declarator)->func_type->attrs = append_attr((yyval.declarator)->func_type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str)));
						           else if ((yyval.declarator)->type) (yyval.declarator)->type->attrs = append_attr((yyval.declarator)->type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str))); }
#line 4947 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 344: /* m_abstract_declarator: %empty  */
#line 1038 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator(NULL); }
#line 4953 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 346: /* abstract_direct_declarator: '(' abstract_declarator_no_direct ')'  */
#line 1044 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); }
#line 4959 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 347: /* abstract_direct_declarator: abstract_direct_declarator array  */
#line 1045 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); (yyval.declarator)->type = append_array((yyval.declarator)->type, (yyvsp[0].expr)); }
#line 4965 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 348: /* abstract_direct_declarator: array  */
#line 1046 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator(NULL); (yyval.declarator)->type = append_array((yyval.declarator)->type, (yyvsp[0].expr)); }
#line 4971 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 349: /* abstract_direct_declarator: '(' m_args ')'  */
#line 1048 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator(NULL);
						  (yyval.declarator)->func_type = append_chain_type((yyval.declarator)->type, type_new_function((yyvsp[-1].var_list)));
						  (yyval.declarator)->type = NULL;
						}
#line 4980 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 350: /* abstract_direct_declarator: abstract_direct_declarator '(' m_args ')'  */
#line 1053 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-3].declarator);
						  (yyval.declarator)->func_type = append_chain_type((yyval.declarator)->type, type_new_function((yyvsp[-1].var_list)));
						  (yyval.declarator)->type = NULL;
						}
#line 4989 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 351: /* any_declarator: '*' m_type_qual_list m_any_declarator  */
#line 1062 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type = append_chain_type((yyval.declarator)->type, type_new_pointer(pointer_default, NULL, (yyvsp[-1].attr_list))); }
#line 4995 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 352: /* any_declarator: callconv m_any_declarator  */
#line 1063 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type->attrs = append_attr((yyval.declarator)->type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str))); }
#line 5001 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 354: /* any_declarator_no_direct: '*' m_type_qual_list m_any_declarator  */
#line 1070 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type = append_chain_type((yyval.declarator)->type, type_new_pointer(pointer_default, NULL, (yyvsp[-1].attr_list))); }
#line 5007 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 355: /* any_declarator_no_direct: callconv m_any_declarator  */
#line 1071 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); (yyval.declarator)->type->attrs = append_attr((yyval.declarator)->type->attrs, make_attrp(ATTR_CALLCONV, (yyvsp[-1].str))); }
#line 5013 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 356: /* m_any_declarator: %empty  */
#line 1075 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator(NULL); }
#line 5019 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 358: /* any_direct_declarator: ident  */
#line 1083 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator((yyvsp[0].var)); }
#line 5025 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 359: /* any_direct_declarator: '(' any_declarator_no_direct ')'  */
#line 1084 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); }
#line 5031 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 360: /* any_direct_declarator: any_direct_declarator array  */
#line 1085 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); (yyval.declarator)->type = append_array((yyval.declarator)->type, (yyvsp[0].expr)); }
#line 5037 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 361: /* any_direct_declarator: array  */
#line 1086 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator(NULL); (yyval.declarator)->type = append_array((yyval.declarator)->type, (yyvsp[0].expr)); }
#line 5043 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 362: /* any_direct_declarator: '(' m_args ')'  */
#line 1088 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = make_declarator(NULL);
						  (yyval.declarator)->func_type = append_chain_type((yyval.declarator)->type, type_new_function((yyvsp[-1].var_list)));
						  (yyval.declarator)->type = NULL;
						}
#line 5052 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 363: /* any_direct_declarator: any_direct_declarator '(' m_args ')'  */
#line 1093 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-3].declarator);
						  (yyval.declarator)->func_type = append_chain_type((yyval.declarator)->type, type_new_function((yyvsp[-1].var_list)));
						  (yyval.declarator)->type = NULL;
						}
#line 5061 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 364: /* declarator_list: declarator  */
#line 1100 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator_list) = append_declarator( NULL, (yyvsp[0].declarator) ); }
#line 5067 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 365: /* declarator_list: declarator_list ',' declarator  */
#line 1101 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator_list) = append_declarator( (yyvsp[-2].declarator_list), (yyvsp[0].declarator) ); }
#line 5073 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 366: /* m_bitfield: %empty  */
#line 1104 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = NULL; }
#line 5079 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 367: /* m_bitfield: ':' expr_const  */
#line 1105 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.expr) = (yyvsp[0].expr); }
#line 5085 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 368: /* struct_declarator: any_declarator m_bitfield  */
#line 1108 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-1].declarator); (yyval.declarator)->bits = (yyvsp[0].expr);
						  if (!(yyval.declarator)->bits && !(yyval.declarator)->var->name)
						    error_loc("unnamed fields are not allowed\n");
						}
#line 5094 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 369: /* struct_declarator_list: struct_declarator  */
#line 1115 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator_list) = append_declarator( NULL, (yyvsp[0].declarator) ); }
#line 5100 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 370: /* struct_declarator_list: struct_declarator_list ',' struct_declarator  */
#line 1117 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator_list) = append_declarator( (yyvsp[-2].declarator_list), (yyvsp[0].declarator) ); }
#line 5106 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 371: /* init_declarator: declarator  */
#line 1121 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[0].declarator); }
#line 5112 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 372: /* init_declarator: declarator '=' expr_const  */
#line 1122 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.declarator) = (yyvsp[-2].declarator); (yyvsp[-2].declarator)->var->eval = (yyvsp[0].expr); }
#line 5118 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 373: /* threading_type: tAPARTMENT  */
#line 1126 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = THREADING_APARTMENT; }
#line 5124 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 374: /* threading_type: tNEUTRAL  */
#line 1127 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = THREADING_NEUTRAL; }
#line 5130 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 375: /* threading_type: tSINGLE  */
#line 1128 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = THREADING_SINGLE; }
#line 5136 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 376: /* threading_type: tFREE  */
#line 1129 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = THREADING_FREE; }
#line 5142 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 377: /* threading_type: tBOTH  */
#line 1130 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = THREADING_BOTH; }
#line 5148 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 378: /* pointer_type: tREF  */
#line 1134 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = FC_RP; }
#line 5154 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 379: /* pointer_type: tUNIQUE  */
#line 1135 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = FC_UP; }
#line 5160 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 380: /* pointer_type: tPTR  */
#line 1136 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = FC_FP; }
#line 5166 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 381: /* structdef: tSTRUCT t_ident '{' fields '}'  */
#line 1139 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_struct((yyvsp[-3].str), current_namespace, TRUE, (yyvsp[-1].var_list)); }
#line 5172 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 382: /* type: tVOID  */
#line 1142 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_void(); }
#line 5178 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 383: /* type: aKNOWNTYPE  */
#line 1143 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = find_type_or_error((yyvsp[0].str), 0); }
#line 5184 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 384: /* type: base_type  */
#line 1144 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); }
#line 5190 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 385: /* type: enumdef  */
#line 1145 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); }
#line 5196 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 386: /* type: tENUM aIDENTIFIER  */
#line 1146 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_enum((yyvsp[0].str), current_namespace, FALSE, NULL); }
#line 5202 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 387: /* type: structdef  */
#line 1147 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); }
#line 5208 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 388: /* type: tSTRUCT aIDENTIFIER  */
#line 1148 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_struct((yyvsp[0].str), current_namespace, FALSE, NULL); }
#line 5214 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 389: /* type: uniondef  */
#line 1149 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = (yyvsp[0].type); }
#line 5220 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 390: /* type: tUNION aIDENTIFIER  */
#line 1150 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_nonencapsulated_union((yyvsp[0].str), FALSE, NULL); }
#line 5226 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 391: /* type: tSAFEARRAY '(' type ')'  */
#line 1151 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = make_safearray((yyvsp[-1].type)); }
#line 5232 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 392: /* typedef: m_attributes tTYPEDEF m_attributes decl_spec declarator_list  */
#line 1155 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyvsp[-4].attr_list) = append_attribs((yyvsp[-4].attr_list), (yyvsp[-2].attr_list));
						  reg_typedefs((yyvsp[-1].declspec), (yyvsp[0].declarator_list), check_typedef_attrs((yyvsp[-4].attr_list)));
						  (yyval.statement) = make_statement_typedef((yyvsp[0].declarator_list));
						}
#line 5241 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 393: /* uniondef: tUNION t_ident '{' ne_union_fields '}'  */
#line 1162 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_nonencapsulated_union((yyvsp[-3].str), TRUE, (yyvsp[-1].var_list)); }
#line 5247 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 394: /* uniondef: tUNION t_ident tSWITCH '(' s_field ')' m_ident '{' cases '}'  */
#line 1165 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.type) = type_new_encapsulated_union((yyvsp[-8].str), (yyvsp[-5].var), (yyvsp[-3].var), (yyvsp[-1].var_list)); }
#line 5253 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 395: /* version: aNUM  */
#line 1169 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = MAKEVERSION((yyvsp[0].num), 0); }
#line 5259 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 396: /* version: aNUM '.' aNUM  */
#line 1170 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = MAKEVERSION((yyvsp[-2].num), (yyvsp[0].num)); }
#line 5265 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 397: /* version: aHEXNUM  */
#line 1171 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = (yyvsp[0].num); }
#line 5271 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 402: /* acf_int_statement: tTYPEDEF acf_attributes aKNOWNTYPE ';'  */
#line 1186 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { type_t *type = find_type_or_error((yyvsp[-1].str), 0);
                                                  type->attrs = append_attr_list(type->attrs, (yyvsp[-2].attr_list));
                                                }
#line 5279 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 403: /* acf_interface: acf_attributes tINTERFACE aKNOWNTYPE '{' acf_int_statements '}'  */
#line 1193 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                {  type_t *iface = find_type_or_error2((yyvsp[-3].str), 0);
                                                   if (type_get_type(iface) != TYPE_INTERFACE)
                                                       error_loc("%s is not an interface\n", iface->name);
                                                   iface->attrs = append_attr_list(iface->attrs, (yyvsp[-5].attr_list));
                                                }
#line 5289 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 404: /* acf_attributes: %empty  */
#line 1201 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = NULL; }
#line 5295 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 405: /* acf_attributes: '[' acf_attribute_list ']'  */
#line 1202 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = (yyvsp[-1].attr_list); }
#line 5301 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 406: /* acf_attribute_list: acf_attribute  */
#line 1206 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = append_attr(NULL, (yyvsp[0].attr)); }
#line 5307 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 407: /* acf_attribute_list: acf_attribute_list ',' acf_attribute  */
#line 1207 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr_list) = append_attr((yyvsp[-2].attr_list), (yyvsp[0].attr)); }
#line 5313 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 408: /* acf_attribute: tALLOCATE '(' allocate_option_list ')'  */
#line 1212 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attrv(ATTR_ALLOCATE, (yyvsp[-1].num)); }
#line 5319 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 409: /* acf_attribute: tENCODE  */
#line 1213 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_ENCODE); }
#line 5325 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 410: /* acf_attribute: tDECODE  */
#line 1214 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_DECODE); }
#line 5331 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 411: /* acf_attribute: tEXPLICITHANDLE  */
#line 1215 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.attr) = make_attr(ATTR_EXPLICIT_HANDLE); }
#line 5337 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 412: /* allocate_option_list: allocate_option  */
#line 1219 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = (yyvsp[0].num); }
#line 5343 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 413: /* allocate_option_list: allocate_option_list ',' allocate_option  */
#line 1221 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = (yyvsp[-2].num) | (yyvsp[0].num); }
#line 5349 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 414: /* allocate_option: tDONTFREE  */
#line 1225 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = FC_DONT_FREE; }
#line 5355 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 415: /* allocate_option: tFREE  */
#line 1226 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = 0; }
#line 5361 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 416: /* allocate_option: tALLNODES  */
#line 1227 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = FC_ALLOCATE_ALL_NODES; }
#line 5367 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;

  case 417: /* allocate_option: tSINGLENODE  */
#line 1228 "/home/moebius/reactos/sdk/tools/widl/parser.y"
                                                { (yyval.num) = 0; }
#line 5373 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"
    break;


#line 5377 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.c"

      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", YY_CAST (yysymbol_kind_t, yyr1[yyn]), &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;

  *++yyvsp = yyval;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
  {
    const int yylhs = yyr1[yyn] - YYNTOKENS;
    const int yyi = yypgoto[yylhs] + *yyssp;
    yystate = (0 <= yyi && yyi <= YYLAST && yycheck[yyi] == *yyssp
               ? yytable[yyi]
               : yydefgoto[yylhs]);
  }

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == YYEMPTY ? YYSYMBOL_YYEMPTY : YYTRANSLATE (yychar);
  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
      {
        yypcontext_t yyctx
          = {yyssp, yytoken};
        char const *yymsgp = YY_("syntax error");
        int yysyntax_error_status;
        yysyntax_error_status = yysyntax_error (&yymsg_alloc, &yymsg, &yyctx);
        if (yysyntax_error_status == 0)
          yymsgp = yymsg;
        else if (yysyntax_error_status == -1)
          {
            if (yymsg != yymsgbuf)
              YYSTACK_FREE (yymsg);
            yymsg = YY_CAST (char *,
                             YYSTACK_ALLOC (YY_CAST (YYSIZE_T, yymsg_alloc)));
            if (yymsg)
              {
                yysyntax_error_status
                  = yysyntax_error (&yymsg_alloc, &yymsg, &yyctx);
                yymsgp = yymsg;
              }
            else
              {
                yymsg = yymsgbuf;
                yymsg_alloc = sizeof yymsgbuf;
                yysyntax_error_status = YYENOMEM;
              }
          }
        yyerror (yymsgp);
        if (yysyntax_error_status == YYENOMEM)
          YYNOMEM;
      }
    }

  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */

      if (yychar <= YYEOF)
        {
          /* Return failure if at end of input.  */
          if (yychar == YYEOF)
            YYABORT;
        }
      else
        {
          yydestruct ("Error: discarding",
                      yytoken, &yylval);
          yychar = YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:
  /* Pacify compilers when the user code never invokes YYERROR and the
     label yyerrorlab therefore never appears in user code.  */
  if (0)
    YYERROR;
  ++yynerrs;

  /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  /* Pop stack until we find a state that shifts the error token.  */
  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
        {
          yyn += YYSYMBOL_YYerror;
          if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == YYSYMBOL_YYerror)
            {
              yyn = yytable[yyn];
              if (0 < yyn)
                break;
            }
        }

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
        YYABORT;


      yydestruct ("Error: popping",
                  YY_ACCESSING_SYMBOL (yystate), yyvsp);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END


  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", YY_ACCESSING_SYMBOL (yyn), yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturnlab;


/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturnlab;


/*-----------------------------------------------------------.
| yyexhaustedlab -- YYNOMEM (memory exhaustion) comes here.  |
`-----------------------------------------------------------*/
yyexhaustedlab:
  yyerror (YY_("memory exhausted"));
  yyresult = 2;
  goto yyreturnlab;


/*----------------------------------------------------------.
| yyreturnlab -- parsing is finished, clean up and return.  |
`----------------------------------------------------------*/
yyreturnlab:
  if (yychar != YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  YY_ACCESSING_SYMBOL (+*yyssp), yyvsp);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif
  if (yymsg != yymsgbuf)
    YYSTACK_FREE (yymsg);
  return yyresult;
}

#line 1230 "/home/moebius/reactos/sdk/tools/widl/parser.y"


static void decl_builtin_basic(const char *name, enum type_basic_type type)
{
  type_t *t = type_new_basic(type);
  reg_type(t, name, NULL, 0);
}

static void decl_builtin_alias(const char *name, type_t *t)
{
  reg_type(type_new_alias(t, name), name, NULL, 0);
}

void init_types(void)
{
  decl_builtin_basic("byte", TYPE_BASIC_BYTE);
  decl_builtin_basic("wchar_t", TYPE_BASIC_WCHAR);
  decl_builtin_basic("float", TYPE_BASIC_FLOAT);
  decl_builtin_basic("double", TYPE_BASIC_DOUBLE);
  decl_builtin_basic("error_status_t", TYPE_BASIC_ERROR_STATUS_T);
  decl_builtin_basic("handle_t", TYPE_BASIC_HANDLE);
  decl_builtin_alias("boolean", type_new_basic(TYPE_BASIC_BYTE));
}

static str_list_t *append_str(str_list_t *list, char *str)
{
    struct str_list_entry_t *entry;

    if (!str) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    entry = xmalloc( sizeof(*entry) );
    entry->str = str;
    list_add_tail( list, &entry->entry );
    return list;
}

static attr_list_t *append_attr(attr_list_t *list, attr_t *attr)
{
    attr_t *attr_existing;
    if (!attr) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    LIST_FOR_EACH_ENTRY(attr_existing, list, attr_t, entry)
        if (attr_existing->type == attr->type)
        {
            parser_warning("duplicate attribute %s\n", get_attr_display_name(attr->type));
            /* use the last attribute, like MIDL does */
            list_remove(&attr_existing->entry);
            break;
        }
    list_add_tail( list, &attr->entry );
    return list;
}

static attr_list_t *move_attr(attr_list_t *dst, attr_list_t *src, enum attr_type type)
{
  attr_t *attr;
  if (!src) return dst;
  LIST_FOR_EACH_ENTRY(attr, src, attr_t, entry)
    if (attr->type == type)
    {
      list_remove(&attr->entry);
      return append_attr(dst, attr);
    }
  return dst;
}

static attr_list_t *append_attr_list(attr_list_t *new_list, attr_list_t *old_list)
{
  struct list *entry;

  if (!old_list) return new_list;

  while ((entry = list_head(old_list)))
  {
    attr_t *attr = LIST_ENTRY(entry, attr_t, entry);
    list_remove(entry);
    new_list = append_attr(new_list, attr);
  }
  return new_list;
}

typedef int (*map_attrs_filter_t)(attr_list_t*,const attr_t*);

static attr_list_t *map_attrs(const attr_list_t *list, map_attrs_filter_t filter)
{
  attr_list_t *new_list;
  const attr_t *attr;
  attr_t *new_attr;

  if (!list) return NULL;

  new_list = xmalloc( sizeof(*list) );
  list_init( new_list );
  LIST_FOR_EACH_ENTRY(attr, list, const attr_t, entry)
  {
    if (filter && !filter(new_list, attr)) continue;
    new_attr = xmalloc(sizeof(*new_attr));
    *new_attr = *attr;
    list_add_tail(new_list, &new_attr->entry);
  }
  return new_list;
}

static decl_spec_t *make_decl_spec(type_t *type, decl_spec_t *left, decl_spec_t *right, attr_t *attr, enum storage_class stgclass)
{
  decl_spec_t *declspec = left ? left : right;
  if (!declspec)
  {
    declspec = xmalloc(sizeof(*declspec));
    declspec->type = NULL;
    declspec->attrs = NULL;
    declspec->stgclass = STG_NONE;
  }
  declspec->type = type;
  if (left && declspec != left)
  {
    declspec->attrs = append_attr_list(declspec->attrs, left->attrs);
    if (declspec->stgclass == STG_NONE)
      declspec->stgclass = left->stgclass;
    else if (left->stgclass != STG_NONE)
      error_loc("only one storage class can be specified\n");
    assert(!left->type);
    free(left);
  }
  if (right && declspec != right)
  {
    declspec->attrs = append_attr_list(declspec->attrs, right->attrs);
    if (declspec->stgclass == STG_NONE)
      declspec->stgclass = right->stgclass;
    else if (right->stgclass != STG_NONE)
      error_loc("only one storage class can be specified\n");
    assert(!right->type);
    free(right);
  }

  declspec->attrs = append_attr(declspec->attrs, attr);
  if (declspec->stgclass == STG_NONE)
    declspec->stgclass = stgclass;
  else if (stgclass != STG_NONE)
    error_loc("only one storage class can be specified\n");

  /* apply attributes to type */
  if (type && declspec->attrs)
  {
    attr_list_t *attrs;
    declspec->type = duptype(type, 1);
    attrs = map_attrs(type->attrs, NULL);
    declspec->type->attrs = append_attr_list(attrs, declspec->attrs);
    declspec->attrs = NULL;
  }

  return declspec;
}

static attr_t *make_attr(enum attr_type type)
{
  attr_t *a = xmalloc(sizeof(attr_t));
  a->type = type;
  a->u.ival = 0;
  return a;
}

static attr_t *make_attrv(enum attr_type type, unsigned int val)
{
  attr_t *a = xmalloc(sizeof(attr_t));
  a->type = type;
  a->u.ival = val;
  return a;
}

static attr_t *make_attrp(enum attr_type type, void *val)
{
  attr_t *a = xmalloc(sizeof(attr_t));
  a->type = type;
  a->u.pval = val;
  return a;
}

static expr_list_t *append_expr(expr_list_t *list, expr_t *expr)
{
    if (!expr) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    list_add_tail( list, &expr->entry );
    return list;
}

static type_t *append_array(type_t *chain, expr_t *expr)
{
    type_t *array;

    if (!expr)
        return chain;

    /* An array is always a reference pointer unless explicitly marked otherwise
     * (regardless of what the default pointer attribute is). */
    array = type_new_array(NULL, NULL, FALSE, expr->is_const ? expr->cval : 0,
            expr->is_const ? NULL : expr, NULL, FC_RP);

    return append_chain_type(chain, array);
}

static struct list type_pool = LIST_INIT(type_pool);
typedef struct
{
  type_t data;
  struct list link;
} type_pool_node_t;

type_t *alloc_type(void)
{
  type_pool_node_t *node = xmalloc(sizeof *node);
  list_add_tail(&type_pool, &node->link);
  return &node->data;
}

void set_all_tfswrite(int val)
{
  type_pool_node_t *node;
  LIST_FOR_EACH_ENTRY(node, &type_pool, type_pool_node_t, link)
    node->data.tfswrite = val;
}

void clear_all_offsets(void)
{
  type_pool_node_t *node;
  LIST_FOR_EACH_ENTRY(node, &type_pool, type_pool_node_t, link)
    node->data.typestring_offset = node->data.ptrdesc = 0;
}

static void type_function_add_head_arg(type_t *type, var_t *arg)
{
    if (!type->details.function->args)
    {
        type->details.function->args = xmalloc( sizeof(*type->details.function->args) );
        list_init( type->details.function->args );
    }
    list_add_head( type->details.function->args, &arg->entry );
}

static int is_allowed_range_type(const type_t *type)
{
    switch (type_get_type(type))
    {
    case TYPE_ENUM:
        return TRUE;
    case TYPE_BASIC:
        switch (type_basic_get_type(type))
        {
        case TYPE_BASIC_INT8:
        case TYPE_BASIC_INT16:
        case TYPE_BASIC_INT32:
        case TYPE_BASIC_INT64:
        case TYPE_BASIC_INT:
        case TYPE_BASIC_INT3264:
        case TYPE_BASIC_LONG:
        case TYPE_BASIC_BYTE:
        case TYPE_BASIC_CHAR:
        case TYPE_BASIC_WCHAR:
        case TYPE_BASIC_HYPER:
            return TRUE;
        case TYPE_BASIC_FLOAT:
        case TYPE_BASIC_DOUBLE:
        case TYPE_BASIC_ERROR_STATUS_T:
        case TYPE_BASIC_HANDLE:
            return FALSE;
        }
        return FALSE;
    default:
        return FALSE;
    }
}

static type_t *get_array_or_ptr_ref(type_t *type)
{
    if (is_ptr(type))
        return type_pointer_get_ref(type);
    else if (is_array(type))
        return type_array_get_element(type);
    return NULL;
}

static type_t *append_chain_type(type_t *chain, type_t *type)
{
    type_t *chain_type;

    if (!chain)
        return type;
    for (chain_type = chain; get_array_or_ptr_ref(chain_type); chain_type = get_array_or_ptr_ref(chain_type))
        ;

    if (is_ptr(chain_type))
        chain_type->details.pointer.ref = type;
    else if (is_array(chain_type))
        chain_type->details.array.elem = type;
    else
        assert(0);

    return chain;
}

static warning_list_t *append_warning(warning_list_t *list, int num)
{
    warning_t *entry;

    if(!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    entry = xmalloc( sizeof(*entry) );
    entry->num = num;
    list_add_tail( list, &entry->entry );
    return list;
}

static var_t *declare_var(attr_list_t *attrs, decl_spec_t *decl_spec, const declarator_t *decl,
                       int top)
{
  var_t *v = decl->var;
  expr_list_t *sizes = get_attrp(attrs, ATTR_SIZEIS);
  expr_list_t *lengs = get_attrp(attrs, ATTR_LENGTHIS);
  expr_t *dim;
  type_t **ptype;
  type_t *func_type = decl ? decl->func_type : NULL;
  type_t *type = decl_spec->type;

  if (is_attr(type->attrs, ATTR_INLINE))
  {
    if (!func_type)
      error_loc("inline attribute applied to non-function type\n");
    else
    {
      type_t *t;
      /* move inline attribute from return type node to function node */
      for (t = func_type; is_ptr(t); t = type_pointer_get_ref(t))
        ;
      t->attrs = move_attr(t->attrs, type->attrs, ATTR_INLINE);
    }
  }

  /* add type onto the end of the pointers in pident->type */
  v->type = append_chain_type(decl ? decl->type : NULL, type);
  v->stgclass = decl_spec->stgclass;
  v->attrs = attrs;

  /* check for pointer attribute being applied to non-pointer, non-array
   * type */
  if (!is_array(v->type))
  {
    int ptr_attr = get_attrv(v->attrs, ATTR_POINTERTYPE);
    const type_t *ptr = NULL;
    /* pointer attributes on the left side of the type belong to the function
     * pointer, if one is being declared */
    type_t **pt = func_type ? &func_type : &v->type;
    for (ptr = *pt; ptr && !ptr_attr; )
    {
      ptr_attr = get_attrv(ptr->attrs, ATTR_POINTERTYPE);
      if (!ptr_attr && type_is_alias(ptr))
        ptr = type_alias_get_aliasee(ptr);
      else
        break;
    }
    if (is_ptr(ptr))
    {
      if (ptr_attr && ptr_attr != FC_UP &&
          type_get_type(type_pointer_get_ref(ptr)) == TYPE_INTERFACE)
          warning_loc_info(&v->loc_info,
                           "%s: pointer attribute applied to interface "
                           "pointer type has no effect\n", v->name);
      if (!ptr_attr && top && (*pt)->details.pointer.def_fc != FC_RP)
      {
        /* FIXME: this is a horrible hack to cope with the issue that we
         * store an offset to the typeformat string in the type object, but
         * two typeformat strings may be written depending on whether the
         * pointer is a toplevel parameter or not */
        *pt = duptype(*pt, 1);
      }
    }
    else if (ptr_attr)
       error_loc("%s: pointer attribute applied to non-pointer type\n", v->name);
  }

  if (is_attr(v->attrs, ATTR_STRING))
  {
    type_t *t = type;

    if (!is_ptr(v->type) && !is_array(v->type))
      error_loc("'%s': [string] attribute applied to non-pointer, non-array type\n",
                v->name);

    for (;;)
    {
        if (is_ptr(t))
            t = type_pointer_get_ref(t);
        else if (is_array(t))
            t = type_array_get_element(t);
        else
            break;
    }

    if (type_get_type(t) != TYPE_BASIC &&
        (get_basic_fc(t) != FC_CHAR &&
         get_basic_fc(t) != FC_BYTE &&
         get_basic_fc(t) != FC_WCHAR))
    {
      error_loc("'%s': [string] attribute is only valid on 'char', 'byte', or 'wchar_t' pointers and arrays\n",
                v->name);
    }
  }

  if (is_attr(v->attrs, ATTR_V1ENUM))
  {
    if (type_get_type_detect_alias(v->type) != TYPE_ENUM)
      error_loc("'%s': [v1_enum] attribute applied to non-enum type\n", v->name);
  }

  if (is_attr(v->attrs, ATTR_RANGE) && !is_allowed_range_type(v->type))
    error_loc("'%s': [range] attribute applied to non-integer type\n",
              v->name);

  ptype = &v->type;
  if (sizes) LIST_FOR_EACH_ENTRY(dim, sizes, expr_t, entry)
  {
    if (dim->type != EXPR_VOID)
    {
      if (is_array(*ptype))
      {
        if (!type_array_get_conformance(*ptype) ||
            type_array_get_conformance(*ptype)->type != EXPR_VOID)
          error_loc("%s: cannot specify size_is for an already sized array\n", v->name);
        else
          *ptype = type_new_array((*ptype)->name,
                                  type_array_get_element(*ptype), FALSE,
                                  0, dim, NULL, 0);
      }
      else if (is_ptr(*ptype))
        *ptype = type_new_array((*ptype)->name, type_pointer_get_ref(*ptype), TRUE,
                                0, dim, NULL, pointer_default);
      else
        error_loc("%s: size_is attribute applied to illegal type\n", v->name);
    }

    if (is_ptr(*ptype))
      ptype = &(*ptype)->details.pointer.ref;
    else if (is_array(*ptype))
      ptype = &(*ptype)->details.array.elem;
    else
      error_loc("%s: too many expressions in size_is attribute\n", v->name);
  }

  ptype = &v->type;
  if (lengs) LIST_FOR_EACH_ENTRY(dim, lengs, expr_t, entry)
  {
    if (dim->type != EXPR_VOID)
    {
      if (is_array(*ptype))
      {
        *ptype = type_new_array((*ptype)->name,
                                type_array_get_element(*ptype),
                                type_array_is_decl_as_ptr(*ptype),
                                type_array_get_dim(*ptype),
                                type_array_get_conformance(*ptype),
                                dim, type_array_get_ptr_default_fc(*ptype));
      }
      else
        error_loc("%s: length_is attribute applied to illegal type\n", v->name);
    }

    if (is_ptr(*ptype))
      ptype = &(*ptype)->details.pointer.ref;
    else if (is_array(*ptype))
      ptype = &(*ptype)->details.array.elem;
    else
      error_loc("%s: too many expressions in length_is attribute\n", v->name);
  }

  /* v->type is currently pointing to the type on the left-side of the
   * declaration, so we need to fix this up so that it is the return type of the
   * function and make v->type point to the function side of the declaration */
  if (func_type)
  {
    type_t *ft, *t;
    type_t *return_type = v->type;
    v->type = func_type;
    for (ft = v->type; is_ptr(ft); ft = type_pointer_get_ref(ft))
      ;
    assert(type_get_type_detect_alias(ft) == TYPE_FUNCTION);
    ft->details.function->retval = make_var(xstrdup("_RetVal"));
    ft->details.function->retval->type = return_type;
    /* move calling convention attribute, if present, from pointer nodes to
     * function node */
    for (t = v->type; is_ptr(t); t = type_pointer_get_ref(t))
      ft->attrs = move_attr(ft->attrs, t->attrs, ATTR_CALLCONV);
  }
  else
  {
    type_t *t;
    for (t = v->type; is_ptr(t); t = type_pointer_get_ref(t))
      if (is_attr(t->attrs, ATTR_CALLCONV))
        error_loc("calling convention applied to non-function-pointer type\n");
  }

  if (decl->bits)
    v->type = type_new_bitfield(v->type, decl->bits);

  return v;
}

static var_list_t *set_var_types(attr_list_t *attrs, decl_spec_t *decl_spec, declarator_list_t *decls)
{
  declarator_t *decl, *next;
  var_list_t *var_list = NULL;

  LIST_FOR_EACH_ENTRY_SAFE( decl, next, decls, declarator_t, entry )
  {
    var_t *var = declare_var(attrs, decl_spec, decl, 0);
    var_list = append_var(var_list, var);
    free(decl);
  }
  free(decl_spec);
  return var_list;
}

static ifref_list_t *append_ifref(ifref_list_t *list, ifref_t *iface)
{
    if (!iface) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    list_add_tail( list, &iface->entry );
    return list;
}

static ifref_t *make_ifref(type_t *iface)
{
  ifref_t *l = xmalloc(sizeof(ifref_t));
  l->iface = iface;
  l->attrs = NULL;
  return l;
}

var_list_t *append_var(var_list_t *list, var_t *var)
{
    if (!var) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    list_add_tail( list, &var->entry );
    return list;
}

static var_list_t *append_var_list(var_list_t *list, var_list_t *vars)
{
    if (!vars) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    list_move_tail( list, vars );
    return list;
}

var_t *make_var(char *name)
{
  var_t *v = xmalloc(sizeof(var_t));
  v->name = name;
  v->type = NULL;
  v->attrs = NULL;
  v->eval = NULL;
  v->stgclass = STG_NONE;
  init_loc_info(&v->loc_info);
  return v;
}

static var_t *copy_var(var_t *src, char *name, map_attrs_filter_t attr_filter)
{
  var_t *v = xmalloc(sizeof(var_t));
  v->name = name;
  v->type = src->type;
  v->attrs = map_attrs(src->attrs, attr_filter);
  v->eval = src->eval;
  v->stgclass = src->stgclass;
  v->loc_info = src->loc_info;
  return v;
}

static declarator_list_t *append_declarator(declarator_list_t *list, declarator_t *d)
{
  if (!d) return list;
  if (!list) {
    list = xmalloc(sizeof(*list));
    list_init(list);
  }
  list_add_tail(list, &d->entry);
  return list;
}

static declarator_t *make_declarator(var_t *var)
{
  declarator_t *d = xmalloc(sizeof(*d));
  d->var = var ? var : make_var(NULL);
  d->type = NULL;
  d->func_type = NULL;
  d->bits = NULL;
  return d;
}

static type_t *make_safearray(type_t *type)
{
  return type_new_array(NULL, type_new_alias(type, "SAFEARRAY"), TRUE, 0,
                        NULL, NULL, FC_RP);
}

static typelib_t *make_library(const char *name, const attr_list_t *attrs)
{
    typelib_t *typelib = xmalloc(sizeof(*typelib));
    typelib->name = xstrdup(name);
    typelib->attrs = attrs;
    list_init( &typelib->importlibs );
    return typelib;
}

static int hash_ident(const char *name)
{
  const char *p = name;
  int sum = 0;
  /* a simple sum hash is probably good enough */
  while (*p) {
    sum += *p;
    p++;
  }
  return sum & (HASHMAX-1);
}

/***** type repository *****/

static struct namespace *find_sub_namespace(struct namespace *namespace, const char *name)
{
  struct namespace *cur;

  LIST_FOR_EACH_ENTRY(cur, &namespace->children, struct namespace, entry) {
    if(!strcmp(cur->name, name))
      return cur;
  }

  return NULL;
}

static void push_namespace(const char *name)
{
  struct namespace *namespace;

  namespace = find_sub_namespace(current_namespace, name);
  if(!namespace) {
    namespace = xmalloc(sizeof(*namespace));
    namespace->name = xstrdup(name);
    namespace->parent = current_namespace;
    list_add_tail(&current_namespace->children, &namespace->entry);
    list_init(&namespace->children);
    memset(namespace->type_hash, 0, sizeof(namespace->type_hash));
  }

  current_namespace = namespace;
}

static void pop_namespace(const char *name)
{
  assert(!strcmp(current_namespace->name, name) && current_namespace->parent);
  current_namespace = current_namespace->parent;
}

struct rtype {
  const char *name;
  type_t *type;
  int t;
  struct rtype *next;
};

type_t *reg_type(type_t *type, const char *name, struct namespace *namespace, int t)
{
  struct rtype *nt;
  int hash;
  if (!name) {
    error_loc("registering named type without name\n");
    return type;
  }
  if (!namespace)
    namespace = &global_namespace;
  hash = hash_ident(name);
  nt = xmalloc(sizeof(struct rtype));
  nt->name = name;
  if (is_global_namespace(namespace))
    type->c_name = name;
  else
    type->c_name = format_namespace(namespace, "__x_", "_C", name);
  nt->type = type;
  nt->t = t;
  nt->next = namespace->type_hash[hash];
  namespace->type_hash[hash] = nt;
  if ((t == tsSTRUCT || t == tsUNION))
    fix_incomplete_types(type);
  return type;
}

static int is_incomplete(const type_t *t)
{
  return !t->defined &&
    (type_get_type_detect_alias(t) == TYPE_STRUCT ||
     type_get_type_detect_alias(t) == TYPE_UNION ||
     type_get_type_detect_alias(t) == TYPE_ENCAPSULATED_UNION);
}

void add_incomplete(type_t *t)
{
  struct typenode *tn = xmalloc(sizeof *tn);
  tn->type = t;
  list_add_tail(&incomplete_types, &tn->entry);
}

static void fix_type(type_t *t)
{
  if (type_is_alias(t) && is_incomplete(t)) {
    type_t *ot = type_alias_get_aliasee(t);
    fix_type(ot);
    if (type_get_type_detect_alias(ot) == TYPE_STRUCT ||
        type_get_type_detect_alias(ot) == TYPE_UNION ||
        type_get_type_detect_alias(ot) == TYPE_ENCAPSULATED_UNION)
      t->details.structure = ot->details.structure;
    t->defined = ot->defined;
  }
}

static void fix_incomplete(void)
{
  struct typenode *tn, *next;

  LIST_FOR_EACH_ENTRY_SAFE(tn, next, &incomplete_types, struct typenode, entry) {
    fix_type(tn->type);
    list_remove(&tn->entry);
    free(tn);
  }
}

static void fix_incomplete_types(type_t *complete_type)
{
  struct typenode *tn, *next;

  LIST_FOR_EACH_ENTRY_SAFE(tn, next, &incomplete_types, struct typenode, entry)
  {
    if (type_is_equal(complete_type, tn->type))
    {
      tn->type->details.structure = complete_type->details.structure;
      list_remove(&tn->entry);
      free(tn);
    }
  }
}

static type_t *reg_typedefs(decl_spec_t *decl_spec, declarator_list_t *decls, attr_list_t *attrs)
{
  const declarator_t *decl;
  type_t *type = decl_spec->type;

  if (is_attr(attrs, ATTR_UUID) && !is_attr(attrs, ATTR_PUBLIC))
    attrs = append_attr( attrs, make_attr(ATTR_PUBLIC) );

  /* We must generate names for tagless enum, struct or union.
     Typedef-ing a tagless enum, struct or union means we want the typedef
     to be included in a library hence the public attribute.  */
  if (type_get_type_detect_alias(type) == TYPE_ENUM ||
      type_get_type_detect_alias(type) == TYPE_STRUCT ||
      type_get_type_detect_alias(type) == TYPE_UNION ||
      type_get_type_detect_alias(type) == TYPE_ENCAPSULATED_UNION)
  {
    if (!type->name)
      type->name = gen_name();

    /* replace existing attributes when generating a typelib */
    if (do_typelib)
        type->attrs = attrs;
  }

#ifdef __REACTOS__ /* r53187 / 5bf224e */
  /* Append the SWITCHTYPE attribute to a non-encapsulated union if it does not already have it.  */
  if (type_get_type_detect_alias(type) == TYPE_UNION &&
      is_attr(attrs, ATTR_SWITCHTYPE) &&
      !is_attr(type->attrs, ATTR_SWITCHTYPE))
    type->attrs = append_attr(type->attrs, make_attrp(ATTR_SWITCHTYPE, get_attrp(attrs, ATTR_SWITCHTYPE)));
#endif

  LIST_FOR_EACH_ENTRY( decl, decls, const declarator_t, entry )
  {

    if (decl->var->name) {
      type_t *cur;
      var_t *name;

      cur = find_type(decl->var->name, current_namespace, 0);

      /*
       * MIDL allows shadowing types that are declared in imported files.
       * We don't throw an error in this case and instead add a new type
       * (which is earlier on the list in hash table, so it will be used
       * instead of shadowed type).
       *
       * FIXME: We may consider string separated type tables for each input
       *        for cleaner solution.
       */
      if (cur && input_name == cur->loc_info.input_name)
          error_loc("%s: redefinition error; original definition was at %s:%d\n",
                    cur->name, cur->loc_info.input_name,
                    cur->loc_info.line_number);

      name = declare_var(attrs, decl_spec, decl, 0);
      cur = type_new_alias(name->type, name->name);
      cur->attrs = attrs;

      if (is_incomplete(cur))
        add_incomplete(cur);
      reg_type(cur, cur->name, current_namespace, 0);
    }
  }
  return type;
}

type_t *find_type(const char *name, struct namespace *namespace, int t)
{
  struct rtype *cur;

  if(namespace && namespace != &global_namespace) {
    for(cur = namespace->type_hash[hash_ident(name)]; cur; cur = cur->next) {
      if(cur->t == t && !strcmp(cur->name, name))
        return cur->type;
    }
  }
  for(cur = global_namespace.type_hash[hash_ident(name)]; cur; cur = cur->next) {
    if(cur->t == t && !strcmp(cur->name, name))
      return cur->type;
  }
  return NULL;
}

static type_t *find_type_or_error(const char *name, int t)
{
  type_t *type = find_type(name, NULL, t);
  if (!type) {
    error_loc("type '%s' not found\n", name);
    return NULL;
  }
  return type;
}

static type_t *find_type_or_error2(char *name, int t)
{
  type_t *tp = find_type_or_error(name, t);
  free(name);
  return tp;
}

int is_type(const char *name)
{
  return find_type(name, current_namespace, 0) != NULL;
}

type_t *get_type(enum type_type type, char *name, struct namespace *namespace, int t)
{
  type_t *tp;
  if (!namespace)
    namespace = &global_namespace;
  if (name) {
    tp = find_type(name, namespace, t);
    if (tp) {
      free(name);
      return tp;
    }
  }
  tp = make_type(type);
  tp->name = name;
  tp->namespace = namespace;
  if (!name) return tp;
  return reg_type(tp, name, namespace, t);
}

/***** constant repository *****/

struct rconst {
  char *name;
  var_t *var;
  struct rconst *next;
};

struct rconst *const_hash[HASHMAX];

static var_t *reg_const(var_t *var)
{
  struct rconst *nc;
  int hash;
  if (!var->name) {
    error_loc("registering constant without name\n");
    return var;
  }
  hash = hash_ident(var->name);
  nc = xmalloc(sizeof(struct rconst));
  nc->name = var->name;
  nc->var = var;
  nc->next = const_hash[hash];
  const_hash[hash] = nc;
  return var;
}

var_t *find_const(const char *name, int f)
{
  struct rconst *cur = const_hash[hash_ident(name)];
  while (cur && strcmp(cur->name, name))
    cur = cur->next;
  if (!cur) {
    if (f) error_loc("constant '%s' not found\n", name);
    return NULL;
  }
  return cur->var;
}

static char *gen_name(void)
{
  static const char format[] = "__WIDL_%s_generated_name_%08lX";
  static unsigned long n = 0;
  static const char *file_id;
  static size_t size;
  char *name;

  if (! file_id)
  {
    char *dst = dup_basename(input_idl_name, ".idl");
    file_id = dst;

    for (; *dst; ++dst)
      if (! isalnum((unsigned char) *dst))
        *dst = '_';

    size = sizeof format - 7 + strlen(file_id) + 8;
  }

  name = xmalloc(size);
  sprintf(name, format, file_id, n++);
  return name;
}

struct allowed_attr
{
    unsigned int dce_compatible : 1;
    unsigned int acf : 1;
    unsigned int on_interface : 1;
    unsigned int on_function : 1;
    unsigned int on_arg : 1;
    unsigned int on_type : 1;
    unsigned int on_enum : 1;
    unsigned int on_struct : 2;
    unsigned int on_union : 1;
    unsigned int on_field : 1;
    unsigned int on_library : 1;
    unsigned int on_dispinterface : 1;
    unsigned int on_module : 1;
    unsigned int on_coclass : 1;
    const char *display_name;
};

struct allowed_attr allowed_attr[] =
{
    /* attr                        { D ACF I Fn ARG T En St Un Fi  L  DI M  C  <display name> } */
    /* ATTR_AGGREGATABLE */        { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "aggregatable" },
    /* ATTR_ALLOCATE */            { 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "allocate" },
    /* ATTR_ANNOTATION */          { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "annotation" },
    /* ATTR_APPOBJECT */           { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "appobject" },
    /* ATTR_ASYNC */               { 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "async" },
    /* ATTR_ASYNCUUID */           { 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, "async_uuid" },
    /* ATTR_AUTO_HANDLE */         { 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "auto_handle" },
    /* ATTR_BINDABLE */            { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "bindable" },
    /* ATTR_BROADCAST */           { 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "broadcast" },
    /* ATTR_CALLAS */              { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "call_as" },
    /* ATTR_CALLCONV */            { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, NULL },
    /* ATTR_CASE */                { 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, "case" },
    /* ATTR_CODE */                { 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "code" },
    /* ATTR_COMMSTATUS */          { 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "comm_status" },
    /* ATTR_CONST */               { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "const" },
    /* ATTR_CONTEXTHANDLE */       { 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "context_handle" },
    /* ATTR_CONTROL */             { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, "control" },
    /* ATTR_DECODE */              { 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "decode" },
    /* ATTR_DEFAULT */             { 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, "default" },
    /* ATTR_DEFAULTBIND */         { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "defaultbind" },
    /* ATTR_DEFAULTCOLLELEM */     { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "defaultcollelem" },
    /* ATTR_DEFAULTVALUE */        { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "defaultvalue" },
    /* ATTR_DEFAULTVTABLE */       { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "defaultvtable" },
 /* ATTR_DISABLECONSISTENCYCHECK */{ 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "disable_consistency_check" },
    /* ATTR_DISPINTERFACE */       { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, NULL },
    /* ATTR_DISPLAYBIND */         { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "displaybind" },
    /* ATTR_DLLNAME */             { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, "dllname" },
    /* ATTR_DUAL */                { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "dual" },
    /* ATTR_ENABLEALLOCATE */      { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "enable_allocate" },
    /* ATTR_ENCODE */              { 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "encode" },
    /* ATTR_ENDPOINT */            { 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "endpoint" },
    /* ATTR_ENTRY */               { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "entry" },
    /* ATTR_EXPLICIT_HANDLE */     { 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "explicit_handle" },
    /* ATTR_FAULTSTATUS */         { 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "fault_status" },
    /* ATTR_FORCEALLOCATE */       { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "force_allocate" },
    /* ATTR_HANDLE */              { 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "handle" },
    /* ATTR_HELPCONTEXT */         { 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, "helpcontext" },
    /* ATTR_HELPFILE */            { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, "helpfile" },
    /* ATTR_HELPSTRING */          { 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, "helpstring" },
    /* ATTR_HELPSTRINGCONTEXT */   { 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, "helpstringcontext" },
    /* ATTR_HELPSTRINGDLL */       { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, "helpstringdll" },
    /* ATTR_HIDDEN */              { 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, "hidden" },
    /* ATTR_ID */                  { 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, "id" },
    /* ATTR_IDEMPOTENT */          { 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "idempotent" },
    /* ATTR_IGNORE */              { 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, "ignore" },
    /* ATTR_IIDIS */               { 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, "iid_is" },
    /* ATTR_IMMEDIATEBIND */       { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "immediatebind" },
    /* ATTR_IMPLICIT_HANDLE */     { 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "implicit_handle" },
    /* ATTR_IN */                  { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "in" },
    /* ATTR_INLINE */              { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "inline" },
    /* ATTR_INPUTSYNC */           { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "inputsync" },
    /* ATTR_LENGTHIS */            { 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, "length_is" },
    /* ATTR_LIBLCID */             { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, "lcid" },
    /* ATTR_LICENSED */            { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "licensed" },
    /* ATTR_LOCAL */               { 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "local" },
    /* ATTR_MAYBE */               { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "maybe" },
    /* ATTR_MESSAGE */             { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "message" },
    /* ATTR_NOCODE */              { 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "nocode" },
    /* ATTR_NONBROWSABLE */        { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "nonbrowsable" },
    /* ATTR_NONCREATABLE */        { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "noncreatable" },
    /* ATTR_NONEXTENSIBLE */       { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "nonextensible" },
    /* ATTR_NOTIFY */              { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "notify" },
    /* ATTR_NOTIFYFLAG */          { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "notify_flag" },
    /* ATTR_OBJECT */              { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "object" },
    /* ATTR_ODL */                 { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, "odl" },
    /* ATTR_OLEAUTOMATION */       { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "oleautomation" },
    /* ATTR_OPTIMIZE */            { 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "optimize" },
    /* ATTR_OPTIONAL */            { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "optional" },
    /* ATTR_OUT */                 { 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "out" },
    /* ATTR_PARAMLCID */           { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "lcid" },
    /* ATTR_PARTIALIGNORE */       { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "partial_ignore" },
    /* ATTR_POINTERDEFAULT */      { 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "pointer_default" },
    /* ATTR_POINTERTYPE */         { 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, "ref, unique or ptr" },
    /* ATTR_PROGID */              { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "progid" },
    /* ATTR_PROPGET */             { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "propget" },
    /* ATTR_PROPPUT */             { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "propput" },
    /* ATTR_PROPPUTREF */          { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "propputref" },
    /* ATTR_PROXY */               { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "proxy" },
    /* ATTR_PUBLIC */              { 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "public" },
    /* ATTR_RANGE */               { 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, "range" },
    /* ATTR_READONLY */            { 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, "readonly" },
    /* ATTR_REPRESENTAS */         { 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "represent_as" },
    /* ATTR_REQUESTEDIT */         { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "requestedit" },
    /* ATTR_RESTRICTED */          { 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, "restricted" },
    /* ATTR_RETVAL */              { 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, "retval" },
    /* ATTR_SIZEIS */              { 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, "size_is" },
    /* ATTR_SOURCE */              { 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, "source" },
    /* ATTR_STRICTCONTEXTHANDLE */ { 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "strict_context_handle" },
    /* ATTR_STRING */              { 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, "string" },
    /* ATTR_SWITCHIS */            { 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, "switch_is" },
    /* ATTR_SWITCHTYPE */          { 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, "switch_type" },
    /* ATTR_THREADING */           { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "threading" },
    /* ATTR_TRANSMITAS */          { 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "transmit_as" },
    /* ATTR_UIDEFAULT */           { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "uidefault" },
    /* ATTR_USESGETLASTERROR */    { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "usesgetlasterror" },
    /* ATTR_USERMARSHAL */         { 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "user_marshal" },
    /* ATTR_UUID */                { 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, "uuid" },
    /* ATTR_V1ENUM */              { 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, "v1_enum" },
    /* ATTR_VARARG */              { 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "vararg" },
    /* ATTR_VERSION */             { 1, 0, 1, 0, 0, 1, 1, 2, 0, 0, 1, 0, 0, 1, "version" },
    /* ATTR_VIPROGID */            { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, "vi_progid" },
    /* ATTR_WIREMARSHAL */         { 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, "wire_marshal" },
};

const char *get_attr_display_name(enum attr_type type)
{
    return allowed_attr[type].display_name;
}

static attr_list_t *check_iface_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_interface)
      error_loc("inapplicable attribute %s for interface %s\n",
                allowed_attr[attr->type].display_name, name);
    if (attr->type == ATTR_IMPLICIT_HANDLE)
    {
        const var_t *var = attr->u.pval;
        if (type_get_type( var->type) == TYPE_BASIC &&
            type_basic_get_type( var->type ) == TYPE_BASIC_HANDLE)
            continue;
        if (is_aliaschain_attr( var->type, ATTR_HANDLE ))
            continue;
      error_loc("attribute %s requires a handle type in interface %s\n",
                allowed_attr[attr->type].display_name, name);
    }
  }
  return attrs;
}

static attr_list_t *check_function_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_function)
      error_loc("inapplicable attribute %s for function %s\n",
                allowed_attr[attr->type].display_name, name);
  }
  return attrs;
}

static void check_arg_attrs(const var_t *arg)
{
  const attr_t *attr;

  if (arg->attrs)
  {
    LIST_FOR_EACH_ENTRY(attr, arg->attrs, const attr_t, entry)
    {
      if (!allowed_attr[attr->type].on_arg)
        error_loc("inapplicable attribute %s for argument %s\n",
                  allowed_attr[attr->type].display_name, arg->name);
    }
  }
}

static attr_list_t *check_typedef_attrs(attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_type)
      error_loc("inapplicable attribute %s for typedef\n",
                allowed_attr[attr->type].display_name);
  }
  return attrs;
}

static attr_list_t *check_enum_attrs(attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_enum)
      error_loc("inapplicable attribute %s for enum\n",
                allowed_attr[attr->type].display_name);
  }
  return attrs;
}

static attr_list_t *check_struct_attrs(attr_list_t *attrs)
{
  int mask = winrt_mode ? 3 : 1;
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!(allowed_attr[attr->type].on_struct & mask))
      error_loc("inapplicable attribute %s for struct\n",
                allowed_attr[attr->type].display_name);
  }
  return attrs;
}

static attr_list_t *check_union_attrs(attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_union)
      error_loc("inapplicable attribute %s for union\n",
                allowed_attr[attr->type].display_name);
  }
  return attrs;
}

static attr_list_t *check_field_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_field)
      error_loc("inapplicable attribute %s for field %s\n",
                allowed_attr[attr->type].display_name, name);
  }
  return attrs;
}

static attr_list_t *check_library_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_library)
      error_loc("inapplicable attribute %s for library %s\n",
                allowed_attr[attr->type].display_name, name);
  }
  return attrs;
}

static attr_list_t *check_dispiface_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_dispinterface)
      error_loc("inapplicable attribute %s for dispinterface %s\n",
                allowed_attr[attr->type].display_name, name);
  }
  return attrs;
}

static attr_list_t *check_module_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_module)
      error_loc("inapplicable attribute %s for module %s\n",
                allowed_attr[attr->type].display_name, name);
  }
  return attrs;
}

static attr_list_t *check_coclass_attrs(const char *name, attr_list_t *attrs)
{
  const attr_t *attr;
  if (!attrs) return attrs;
  LIST_FOR_EACH_ENTRY(attr, attrs, const attr_t, entry)
  {
    if (!allowed_attr[attr->type].on_coclass)
      error_loc("inapplicable attribute %s for coclass %s\n",
                allowed_attr[attr->type].display_name, name);
  }
  return attrs;
}

static int is_allowed_conf_type(const type_t *type)
{
    switch (type_get_type(type))
    {
    case TYPE_ENUM:
        return TRUE;
    case TYPE_BASIC:
        switch (type_basic_get_type(type))
        {
        case TYPE_BASIC_INT8:
        case TYPE_BASIC_INT16:
        case TYPE_BASIC_INT32:
        case TYPE_BASIC_INT64:
        case TYPE_BASIC_INT:
        case TYPE_BASIC_LONG:
        case TYPE_BASIC_CHAR:
        case TYPE_BASIC_HYPER:
        case TYPE_BASIC_BYTE:
        case TYPE_BASIC_WCHAR:
            return TRUE;
        default:
            return FALSE;
        }
    case TYPE_ALIAS:
        /* shouldn't get here because of type_get_type call above */
        assert(0);
        /* fall through */
    case TYPE_STRUCT:
    case TYPE_UNION:
    case TYPE_ENCAPSULATED_UNION:
    case TYPE_ARRAY:
    case TYPE_POINTER:
    case TYPE_VOID:
    case TYPE_MODULE:
    case TYPE_COCLASS:
    case TYPE_FUNCTION:
    case TYPE_INTERFACE:
    case TYPE_BITFIELD:
        return FALSE;
    }
    return FALSE;
}

static int is_ptr_guid_type(const type_t *type)
{
    /* first, make sure it is a pointer to something */
    if (!is_ptr(type)) return FALSE;

    /* second, make sure it is a pointer to something of size sizeof(GUID),
     * i.e. 16 bytes */
    return (type_memsize(type_pointer_get_ref(type)) == 16);
}

static void check_conformance_expr_list(const char *attr_name, const var_t *arg, const type_t *container_type, expr_list_t *expr_list)
{
    expr_t *dim;
    struct expr_loc expr_loc;
    expr_loc.v = arg;
    expr_loc.attr = attr_name;
    if (expr_list) LIST_FOR_EACH_ENTRY(dim, expr_list, expr_t, entry)
    {
        if (dim->type != EXPR_VOID)
        {
            const type_t *expr_type = expr_resolve_type(&expr_loc, container_type, dim);
            if (!is_allowed_conf_type(expr_type))
                error_loc_info(&arg->loc_info, "expression must resolve to integral type <= 32bits for attribute %s\n",
                               attr_name);
        }
    }
}

static void check_remoting_fields(const var_t *var, type_t *type);

/* checks that properties common to fields and arguments are consistent */
static void check_field_common(const type_t *container_type,
                               const char *container_name, const var_t *arg)
{
    type_t *type = arg->type;
    int more_to_do;
    const char *container_type_name;
    const char *var_type;

    switch (type_get_type(container_type))
    {
    case TYPE_STRUCT:
        container_type_name = "struct";
        var_type = "field";
        break;
    case TYPE_UNION:
        container_type_name = "union";
        var_type = "arm";
        break;
    case TYPE_ENCAPSULATED_UNION:
        container_type_name = "encapsulated union";
        var_type = "arm";
        break;
    case TYPE_FUNCTION:
        container_type_name = "function";
        var_type = "parameter";
        break;
    default:
        /* should be no other container types */
        assert(0);
        return;
    }

    if (is_attr(arg->attrs, ATTR_LENGTHIS) &&
        (is_attr(arg->attrs, ATTR_STRING) || is_aliaschain_attr(arg->type, ATTR_STRING)))
        error_loc_info(&arg->loc_info,
                       "string and length_is specified for argument %s are mutually exclusive attributes\n",
                       arg->name);

    if (is_attr(arg->attrs, ATTR_SIZEIS))
    {
        expr_list_t *size_is_exprs = get_attrp(arg->attrs, ATTR_SIZEIS);
        check_conformance_expr_list("size_is", arg, container_type, size_is_exprs);
    }
    if (is_attr(arg->attrs, ATTR_LENGTHIS))
    {
        expr_list_t *length_is_exprs = get_attrp(arg->attrs, ATTR_LENGTHIS);
        check_conformance_expr_list("length_is", arg, container_type, length_is_exprs);
    }
    if (is_attr(arg->attrs, ATTR_IIDIS))
    {
        struct expr_loc expr_loc;
        expr_t *expr = get_attrp(arg->attrs, ATTR_IIDIS);
        if (expr->type != EXPR_VOID)
        {
            const type_t *expr_type;
            expr_loc.v = arg;
            expr_loc.attr = "iid_is";
            expr_type = expr_resolve_type(&expr_loc, container_type, expr);
            if (!expr_type || !is_ptr_guid_type(expr_type))
                error_loc_info(&arg->loc_info, "expression must resolve to pointer to GUID type for attribute iid_is\n");
        }
    }
    if (is_attr(arg->attrs, ATTR_SWITCHIS))
    {
        struct expr_loc expr_loc;
        expr_t *expr = get_attrp(arg->attrs, ATTR_SWITCHIS);
        if (expr->type != EXPR_VOID)
        {
            const type_t *expr_type;
            expr_loc.v = arg;
            expr_loc.attr = "switch_is";
            expr_type = expr_resolve_type(&expr_loc, container_type, expr);
            if (!expr_type || !is_allowed_conf_type(expr_type))
                error_loc_info(&arg->loc_info, "expression must resolve to integral type <= 32bits for attribute %s\n",
                               expr_loc.attr);
        }
    }

    do
    {
        more_to_do = FALSE;

        switch (typegen_detect_type(type, arg->attrs, TDT_IGNORE_STRINGS))
        {
        case TGT_STRUCT:
        case TGT_UNION:
            check_remoting_fields(arg, type);
            break;
        case TGT_INVALID:
        {
            const char *reason = "is invalid";
            switch (type_get_type(type))
            {
            case TYPE_VOID:
                reason = "cannot derive from void *";
                break;
            case TYPE_FUNCTION:
                reason = "cannot be a function pointer";
                break;
            case TYPE_BITFIELD:
                reason = "cannot be a bit-field";
                break;
            case TYPE_COCLASS:
                reason = "cannot be a class";
                break;
            case TYPE_INTERFACE:
                reason = "cannot be a non-pointer to an interface";
                break;
            case TYPE_MODULE:
                reason = "cannot be a module";
                break;
            default:
                break;
            }
            error_loc_info(&arg->loc_info, "%s \'%s\' of %s \'%s\' %s\n",
                           var_type, arg->name, container_type_name, container_name, reason);
            break;
        }
        case TGT_CTXT_HANDLE:
        case TGT_CTXT_HANDLE_POINTER:
            if (type_get_type(container_type) != TYPE_FUNCTION)
                error_loc_info(&arg->loc_info,
                               "%s \'%s\' of %s \'%s\' cannot be a context handle\n",
                               var_type, arg->name, container_type_name,
                               container_name);
            break;
        case TGT_STRING:
        {
            const type_t *t = type;
            while (is_ptr(t))
                t = type_pointer_get_ref(t);
            if (is_aliaschain_attr(t, ATTR_RANGE))
                warning_loc_info(&arg->loc_info, "%s: range not verified for a string of ranged types\n", arg->name);
            break;
        }
        case TGT_POINTER:
            type = type_pointer_get_ref(type);
            more_to_do = TRUE;
            break;
        case TGT_ARRAY:
            type = type_array_get_element(type);
            more_to_do = TRUE;
            break;
        case TGT_USER_TYPE:
        case TGT_IFACE_POINTER:
        case TGT_BASIC:
        case TGT_ENUM:
        case TGT_RANGE:
            /* nothing to do */
            break;
        }
    } while (more_to_do);
}

static void check_remoting_fields(const var_t *var, type_t *type)
{
    const var_t *field;
    const var_list_t *fields = NULL;

    type = type_get_real_type(type);

    if (type->checked)
        return;

    type->checked = TRUE;

    if (type_get_type(type) == TYPE_STRUCT)
    {
        if (type_is_complete(type))
            fields = type_struct_get_fields(type);
        else
            error_loc_info(&var->loc_info, "undefined type declaration %s\n", type->name);
    }
    else if (type_get_type(type) == TYPE_UNION || type_get_type(type) == TYPE_ENCAPSULATED_UNION)
        fields = type_union_get_cases(type);

    if (fields) LIST_FOR_EACH_ENTRY( field, fields, const var_t, entry )
        if (field->type) check_field_common(type, type->name, field);
}

/* checks that arguments for a function make sense for marshalling and unmarshalling */
static void check_remoting_args(const var_t *func)
{
    const char *funcname = func->name;
    const var_t *arg;

    if (func->type->details.function->args) LIST_FOR_EACH_ENTRY( arg, func->type->details.function->args, const var_t, entry )
    {
        const type_t *type = arg->type;

        /* check that [out] parameters have enough pointer levels */
        if (is_attr(arg->attrs, ATTR_OUT))
        {
            switch (typegen_detect_type(type, arg->attrs, TDT_ALL_TYPES))
            {
            case TGT_BASIC:
            case TGT_ENUM:
            case TGT_RANGE:
            case TGT_STRUCT:
            case TGT_UNION:
            case TGT_CTXT_HANDLE:
            case TGT_USER_TYPE:
                error_loc_info(&arg->loc_info, "out parameter \'%s\' of function \'%s\' is not a pointer\n", arg->name, funcname);
                break;
            case TGT_IFACE_POINTER:
                error_loc_info(&arg->loc_info, "out interface pointer \'%s\' of function \'%s\' is not a double pointer\n", arg->name, funcname);
                break;
            case TGT_STRING:
                if (is_array(type))
                {
                    /* needs conformance or fixed dimension */
                    if (type_array_has_conformance(type) &&
                        type_array_get_conformance(type)->type != EXPR_VOID) break;
                    if (!type_array_has_conformance(type) && type_array_get_dim(type)) break;
                }
                if (is_attr( arg->attrs, ATTR_IN )) break;
                error_loc_info(&arg->loc_info, "out parameter \'%s\' of function \'%s\' cannot be an unsized string\n", arg->name, funcname);
                break;
            case TGT_INVALID:
                /* already error'd before we get here */
            case TGT_CTXT_HANDLE_POINTER:
            case TGT_POINTER:
            case TGT_ARRAY:
                /* OK */
                break;
            }
        }

        check_field_common(func->type, funcname, arg);
    }

    if (type_get_type(type_function_get_rettype(func->type)) != TYPE_VOID)
    {
        var_t var;
        var = *func;
        var.type = type_function_get_rettype(func->type);
        var.name = xstrdup("return value");
        check_field_common(func->type, funcname, &var);
        free(var.name);
    }
}

static void add_explicit_handle_if_necessary(const type_t *iface, var_t *func)
{
    unsigned char explicit_fc, implicit_fc;

    /* check for a defined binding handle */
    if (!get_func_handle_var( iface, func, &explicit_fc, &implicit_fc ) || !explicit_fc)
    {
        /* no explicit handle specified so add
         * "[in] handle_t IDL_handle" as the first parameter to the
         * function */
        var_t *idl_handle = make_var(xstrdup("IDL_handle"));
        idl_handle->attrs = append_attr(NULL, make_attr(ATTR_IN));
        idl_handle->type = find_type_or_error("handle_t", 0);
        type_function_add_head_arg(func->type, idl_handle);
    }
}

static void check_functions(const type_t *iface, int is_inside_library)
{
    const statement_t *stmt;
    if (is_attr(iface->attrs, ATTR_EXPLICIT_HANDLE))
    {
        STATEMENTS_FOR_EACH_FUNC( stmt, type_iface_get_stmts(iface) )
        {
            var_t *func = stmt->u.var;
            add_explicit_handle_if_necessary(iface, func);
        }
    }
    if (!is_inside_library && !is_attr(iface->attrs, ATTR_LOCAL))
    {
        STATEMENTS_FOR_EACH_FUNC( stmt, type_iface_get_stmts(iface) )
        {
            const var_t *func = stmt->u.var;
            if (!is_attr(func->attrs, ATTR_LOCAL))
                check_remoting_args(func);
        }
    }
}

static char *concat_str(const char *prefix, const char *str)
{
    char *ret = xmalloc(strlen(prefix) + strlen(str) + 1);
    strcpy(ret, prefix);
    strcat(ret, str);
    return ret;
}

static int async_iface_attrs(attr_list_t *attrs, const attr_t *attr)
{
    switch(attr->type)
    {
    case ATTR_UUID:
        return 0;
    case ATTR_ASYNCUUID:
        append_attr(attrs, make_attrp(ATTR_UUID, attr->u.pval));
        return 0;
    default:
        return 1;
    }
}

static int arg_in_attrs(attr_list_t *attrs, const attr_t *attr)
{
    return attr->type != ATTR_OUT && attr->type != ATTR_RETVAL;
}

static int arg_out_attrs(attr_list_t *attrs, const attr_t *attr)
{
    return attr->type != ATTR_IN;
}

static void check_async_uuid(type_t *iface)
{
    statement_list_t *stmts = NULL;
    statement_t *stmt;
    type_t *async_iface;
    type_t *inherit;

    if (!is_attr(iface->attrs, ATTR_ASYNCUUID)) return;

    inherit = iface->details.iface->inherit;
    if (inherit && strcmp(inherit->name, "IUnknown"))
        inherit = inherit->details.iface->async_iface;
    if (!inherit)
        error_loc("async_uuid applied to an interface with incompatible parent\n");

    async_iface = get_type(TYPE_INTERFACE, concat_str("Async", iface->name), iface->namespace, 0);
    async_iface->attrs = map_attrs(iface->attrs, async_iface_attrs);

    STATEMENTS_FOR_EACH_FUNC( stmt, type_iface_get_stmts(iface) )
    {
        var_t *begin_func, *finish_func, *func = stmt->u.var, *arg;
        var_list_t *begin_args = NULL, *finish_args = NULL, *args;

        args = func->type->details.function->args;
        if (args) LIST_FOR_EACH_ENTRY(arg, args, var_t, entry)
        {
            if (is_attr(arg->attrs, ATTR_IN) || !is_attr(arg->attrs, ATTR_OUT))
                begin_args = append_var(begin_args, copy_var(arg, strdup(arg->name), arg_in_attrs));
            if (is_attr(arg->attrs, ATTR_OUT))
                finish_args = append_var(finish_args, copy_var(arg, strdup(arg->name), arg_out_attrs));
        }

        begin_func = copy_var(func, concat_str("Begin_", func->name), NULL);
        begin_func->type = type_new_function(begin_args);
        begin_func->type->attrs = func->attrs;
        begin_func->type->details.function->retval = func->type->details.function->retval;
        stmts = append_statement(stmts, make_statement_declaration(begin_func));

        finish_func = copy_var(func, concat_str("Finish_", func->name), NULL);
        finish_func->type = type_new_function(finish_args);
        finish_func->type->attrs = func->attrs;
        finish_func->type->details.function->retval = func->type->details.function->retval;
        stmts = append_statement(stmts, make_statement_declaration(finish_func));
    }

    type_interface_define(async_iface, inherit, stmts);
    iface->details.iface->async_iface = async_iface->details.iface->async_iface = async_iface;
}

static void check_statements(const statement_list_t *stmts, int is_inside_library)
{
    const statement_t *stmt;

    if (stmts) LIST_FOR_EACH_ENTRY(stmt, stmts, const statement_t, entry)
    {
        switch(stmt->type) {
        case STMT_LIBRARY:
            check_statements(stmt->u.lib->stmts, TRUE);
            break;
        case STMT_TYPE:
            switch(type_get_type(stmt->u.type)) {
            case TYPE_INTERFACE:
                check_functions(stmt->u.type, is_inside_library);
                break;
            case TYPE_COCLASS:
                if(winrt_mode)
                    error_loc("coclass is not allowed in Windows Runtime mode\n");
                break;
            default:
                break;
            }
            break;
        default:
            break;
        }
    }
}

static void check_all_user_types(const statement_list_t *stmts)
{
  const statement_t *stmt;

  if (stmts) LIST_FOR_EACH_ENTRY(stmt, stmts, const statement_t, entry)
  {
    if (stmt->type == STMT_LIBRARY)
      check_all_user_types(stmt->u.lib->stmts);
    else if (stmt->type == STMT_TYPE && type_get_type(stmt->u.type) == TYPE_INTERFACE &&
             !is_local(stmt->u.type->attrs))
    {
      const statement_t *stmt_func;
      STATEMENTS_FOR_EACH_FUNC(stmt_func, type_iface_get_stmts(stmt->u.type)) {
        const var_t *func = stmt_func->u.var;
        check_for_additional_prototype_types(func->type->details.function->args);
      }
    }
  }
}

int is_valid_uuid(const char *s)
{
  int i;

  for (i = 0; i < 36; ++i)
    if (i == 8 || i == 13 || i == 18 || i == 23)
    {
      if (s[i] != '-')
        return FALSE;
    }
    else
      if (!isxdigit(s[i]))
        return FALSE;

  return s[i] == '\0';
}

static statement_t *make_statement(enum statement_type type)
{
    statement_t *stmt = xmalloc(sizeof(*stmt));
    stmt->type = type;
    return stmt;
}

static statement_t *make_statement_type_decl(type_t *type)
{
    statement_t *stmt = make_statement(STMT_TYPE);
    stmt->u.type = type;
    return stmt;
}

static statement_t *make_statement_reference(type_t *type)
{
    statement_t *stmt = make_statement(STMT_TYPEREF);
    stmt->u.type = type;
    return stmt;
}

static statement_t *make_statement_declaration(var_t *var)
{
    statement_t *stmt = make_statement(STMT_DECLARATION);
    stmt->u.var = var;
    if (var->stgclass == STG_EXTERN && var->eval)
        warning("'%s' initialised and declared extern\n", var->name);
    if (is_const_decl(var))
    {
        if (var->eval)
            reg_const(var);
    }
    else if (type_get_type(var->type) == TYPE_FUNCTION)
        check_function_attrs(var->name, var->attrs);
    else if (var->stgclass == STG_NONE || var->stgclass == STG_REGISTER)
        error_loc("instantiation of data is illegal\n");
    return stmt;
}

static statement_t *make_statement_library(typelib_t *typelib)
{
    statement_t *stmt = make_statement(STMT_LIBRARY);
    stmt->u.lib = typelib;
    return stmt;
}

static statement_t *make_statement_pragma(const char *str)
{
    statement_t *stmt = make_statement(STMT_PRAGMA);
    stmt->u.str = str;
    return stmt;
}

static statement_t *make_statement_cppquote(const char *str)
{
    statement_t *stmt = make_statement(STMT_CPPQUOTE);
    stmt->u.str = str;
    return stmt;
}

static statement_t *make_statement_importlib(const char *str)
{
    statement_t *stmt = make_statement(STMT_IMPORTLIB);
    stmt->u.str = str;
    return stmt;
}

static statement_t *make_statement_import(const char *str)
{
    statement_t *stmt = make_statement(STMT_IMPORT);
    stmt->u.str = str;
    return stmt;
}

static statement_t *make_statement_module(type_t *type)
{
    statement_t *stmt = make_statement(STMT_MODULE);
    stmt->u.type = type;
    return stmt;
}

static statement_t *make_statement_typedef(declarator_list_t *decls)
{
    declarator_t *decl, *next;
    statement_t *stmt;
    type_list_t **type_list;

    if (!decls) return NULL;

    stmt = make_statement(STMT_TYPEDEF);
    stmt->u.type_list = NULL;
    type_list = &stmt->u.type_list;

    LIST_FOR_EACH_ENTRY_SAFE( decl, next, decls, declarator_t, entry )
    {
        var_t *var = decl->var;
        type_t *type = find_type_or_error(var->name, 0);
        *type_list = xmalloc(sizeof(type_list_t));
        (*type_list)->type = type;
        (*type_list)->next = NULL;

        type_list = &(*type_list)->next;
        free(decl);
        free(var);
    }

    return stmt;
}

static statement_list_t *append_statements(statement_list_t *l1, statement_list_t *l2)
{
    if (!l2) return l1;
    if (!l1 || l1 == l2) return l2;
    list_move_tail (l1, l2);
    return l1;
}

static attr_list_t *append_attribs(attr_list_t *l1, attr_list_t *l2)
{
    if (!l2) return l1;
    if (!l1 || l1 == l2) return l2;
    list_move_tail (l1, l2);
    return l1;
}

static statement_list_t *append_statement(statement_list_t *list, statement_t *stmt)
{
    if (!stmt) return list;
    if (!list)
    {
        list = xmalloc( sizeof(*list) );
        list_init( list );
    }
    list_add_tail( list, &stmt->entry );
    return list;
}

void init_loc_info(loc_info_t *i)
{
    i->input_name = input_name ? input_name : "stdin";
    i->line_number = line_number;
    i->near_text = parser_text;
}

static void check_def(const type_t *t)
{
    if (t->defined)
        error_loc("%s: redefinition error; original definition was at %s:%d\n",
                  t->name, t->loc_info.input_name, t->loc_info.line_number);
}
