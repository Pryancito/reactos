/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Bison interface for Yacc-like parsers in C

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

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

#ifndef YY_PARSER_HOME_MOEBIUS_REACTOS_OUTPUT_POSIX_AMD64_HOST_TOOLS_BIN_SDK_TOOLS_WIDL_PARSER_TAB_H_INCLUDED
# define YY_PARSER_HOME_MOEBIUS_REACTOS_OUTPUT_POSIX_AMD64_HOST_TOOLS_BIN_SDK_TOOLS_WIDL_PARSER_TAB_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int parser_debug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    YYEOF = 0,                     /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    aIDENTIFIER = 258,             /* aIDENTIFIER  */
    aPRAGMA = 259,                 /* aPRAGMA  */
    aKNOWNTYPE = 260,              /* aKNOWNTYPE  */
    aNUM = 261,                    /* aNUM  */
    aHEXNUM = 262,                 /* aHEXNUM  */
    aDOUBLE = 263,                 /* aDOUBLE  */
    aSTRING = 264,                 /* aSTRING  */
    aWSTRING = 265,                /* aWSTRING  */
    aSQSTRING = 266,               /* aSQSTRING  */
    aUUID = 267,                   /* aUUID  */
    aEOF = 268,                    /* aEOF  */
    aACF = 269,                    /* aACF  */
    SHL = 270,                     /* SHL  */
    SHR = 271,                     /* SHR  */
    MEMBERPTR = 272,               /* MEMBERPTR  */
    EQUALITY = 273,                /* EQUALITY  */
    INEQUALITY = 274,              /* INEQUALITY  */
    GREATEREQUAL = 275,            /* GREATEREQUAL  */
    LESSEQUAL = 276,               /* LESSEQUAL  */
    LOGICALOR = 277,               /* LOGICALOR  */
    LOGICALAND = 278,              /* LOGICALAND  */
    ELLIPSIS = 279,                /* ELLIPSIS  */
    tAGGREGATABLE = 280,           /* tAGGREGATABLE  */
    tALLNODES = 281,               /* tALLNODES  */
    tALLOCATE = 282,               /* tALLOCATE  */
    tANNOTATION = 283,             /* tANNOTATION  */
    tAPPOBJECT = 284,              /* tAPPOBJECT  */
    tASYNC = 285,                  /* tASYNC  */
    tASYNCUUID = 286,              /* tASYNCUUID  */
    tAUTOHANDLE = 287,             /* tAUTOHANDLE  */
    tBINDABLE = 288,               /* tBINDABLE  */
    tBOOLEAN = 289,                /* tBOOLEAN  */
    tBROADCAST = 290,              /* tBROADCAST  */
    tBYTE = 291,                   /* tBYTE  */
    tBYTECOUNT = 292,              /* tBYTECOUNT  */
    tCALLAS = 293,                 /* tCALLAS  */
    tCALLBACK = 294,               /* tCALLBACK  */
    tCASE = 295,                   /* tCASE  */
    tCDECL = 296,                  /* tCDECL  */
    tCHAR = 297,                   /* tCHAR  */
    tCOCLASS = 298,                /* tCOCLASS  */
    tCODE = 299,                   /* tCODE  */
    tCOMMSTATUS = 300,             /* tCOMMSTATUS  */
    tCONST = 301,                  /* tCONST  */
    tCONTEXTHANDLE = 302,          /* tCONTEXTHANDLE  */
    tCONTEXTHANDLENOSERIALIZE = 303, /* tCONTEXTHANDLENOSERIALIZE  */
    tCONTEXTHANDLESERIALIZE = 304, /* tCONTEXTHANDLESERIALIZE  */
    tCONTROL = 305,                /* tCONTROL  */
    tCPPQUOTE = 306,               /* tCPPQUOTE  */
    tDECODE = 307,                 /* tDECODE  */
    tDEFAULT = 308,                /* tDEFAULT  */
    tDEFAULTBIND = 309,            /* tDEFAULTBIND  */
    tDEFAULTCOLLELEM = 310,        /* tDEFAULTCOLLELEM  */
    tDEFAULTVALUE = 311,           /* tDEFAULTVALUE  */
    tDEFAULTVTABLE = 312,          /* tDEFAULTVTABLE  */
    tDISABLECONSISTENCYCHECK = 313, /* tDISABLECONSISTENCYCHECK  */
    tDISPLAYBIND = 314,            /* tDISPLAYBIND  */
    tDISPINTERFACE = 315,          /* tDISPINTERFACE  */
    tDLLNAME = 316,                /* tDLLNAME  */
    tDONTFREE = 317,               /* tDONTFREE  */
    tDOUBLE = 318,                 /* tDOUBLE  */
    tDUAL = 319,                   /* tDUAL  */
    tENABLEALLOCATE = 320,         /* tENABLEALLOCATE  */
    tENCODE = 321,                 /* tENCODE  */
    tENDPOINT = 322,               /* tENDPOINT  */
    tENTRY = 323,                  /* tENTRY  */
    tENUM = 324,                   /* tENUM  */
    tERRORSTATUST = 325,           /* tERRORSTATUST  */
    tEXPLICITHANDLE = 326,         /* tEXPLICITHANDLE  */
    tEXTERN = 327,                 /* tEXTERN  */
    tFALSE = 328,                  /* tFALSE  */
    tFASTCALL = 329,               /* tFASTCALL  */
    tFAULTSTATUS = 330,            /* tFAULTSTATUS  */
    tFLOAT = 331,                  /* tFLOAT  */
    tFORCEALLOCATE = 332,          /* tFORCEALLOCATE  */
    tHANDLE = 333,                 /* tHANDLE  */
    tHANDLET = 334,                /* tHANDLET  */
    tHELPCONTEXT = 335,            /* tHELPCONTEXT  */
    tHELPFILE = 336,               /* tHELPFILE  */
    tHELPSTRING = 337,             /* tHELPSTRING  */
    tHELPSTRINGCONTEXT = 338,      /* tHELPSTRINGCONTEXT  */
    tHELPSTRINGDLL = 339,          /* tHELPSTRINGDLL  */
    tHIDDEN = 340,                 /* tHIDDEN  */
    tHYPER = 341,                  /* tHYPER  */
    tID = 342,                     /* tID  */
    tIDEMPOTENT = 343,             /* tIDEMPOTENT  */
    tIGNORE = 344,                 /* tIGNORE  */
    tIIDIS = 345,                  /* tIIDIS  */
    tIMMEDIATEBIND = 346,          /* tIMMEDIATEBIND  */
    tIMPLICITHANDLE = 347,         /* tIMPLICITHANDLE  */
    tIMPORT = 348,                 /* tIMPORT  */
    tIMPORTLIB = 349,              /* tIMPORTLIB  */
    tIN = 350,                     /* tIN  */
    tIN_LINE = 351,                /* tIN_LINE  */
    tINLINE = 352,                 /* tINLINE  */
    tINPUTSYNC = 353,              /* tINPUTSYNC  */
    tINT = 354,                    /* tINT  */
    tINT32 = 355,                  /* tINT32  */
    tINT3264 = 356,                /* tINT3264  */
    tINT64 = 357,                  /* tINT64  */
    tINTERFACE = 358,              /* tINTERFACE  */
    tLCID = 359,                   /* tLCID  */
    tLENGTHIS = 360,               /* tLENGTHIS  */
    tLIBRARY = 361,                /* tLIBRARY  */
    tLICENSED = 362,               /* tLICENSED  */
    tLOCAL = 363,                  /* tLOCAL  */
    tLONG = 364,                   /* tLONG  */
    tMAYBE = 365,                  /* tMAYBE  */
    tMESSAGE = 366,                /* tMESSAGE  */
    tMETHODS = 367,                /* tMETHODS  */
    tMODULE = 368,                 /* tMODULE  */
    tNAMESPACE = 369,              /* tNAMESPACE  */
    tNOCODE = 370,                 /* tNOCODE  */
    tNONBROWSABLE = 371,           /* tNONBROWSABLE  */
    tNONCREATABLE = 372,           /* tNONCREATABLE  */
    tNONEXTENSIBLE = 373,          /* tNONEXTENSIBLE  */
    tNOTIFY = 374,                 /* tNOTIFY  */
    tNOTIFYFLAG = 375,             /* tNOTIFYFLAG  */
    tNULL = 376,                   /* tNULL  */
    tOBJECT = 377,                 /* tOBJECT  */
    tODL = 378,                    /* tODL  */
    tOLEAUTOMATION = 379,          /* tOLEAUTOMATION  */
    tOPTIMIZE = 380,               /* tOPTIMIZE  */
    tOPTIONAL = 381,               /* tOPTIONAL  */
    tOUT = 382,                    /* tOUT  */
    tPARTIALIGNORE = 383,          /* tPARTIALIGNORE  */
    tPASCAL = 384,                 /* tPASCAL  */
    tPOINTERDEFAULT = 385,         /* tPOINTERDEFAULT  */
    tPRAGMA_WARNING = 386,         /* tPRAGMA_WARNING  */
    tPROGID = 387,                 /* tPROGID  */
    tPROPERTIES = 388,             /* tPROPERTIES  */
    tPROPGET = 389,                /* tPROPGET  */
    tPROPPUT = 390,                /* tPROPPUT  */
    tPROPPUTREF = 391,             /* tPROPPUTREF  */
    tPROXY = 392,                  /* tPROXY  */
    tPTR = 393,                    /* tPTR  */
    tPUBLIC = 394,                 /* tPUBLIC  */
    tRANGE = 395,                  /* tRANGE  */
    tREADONLY = 396,               /* tREADONLY  */
    tREF = 397,                    /* tREF  */
    tREGISTER = 398,               /* tREGISTER  */
    tREPRESENTAS = 399,            /* tREPRESENTAS  */
    tREQUESTEDIT = 400,            /* tREQUESTEDIT  */
    tRESTRICTED = 401,             /* tRESTRICTED  */
    tRETVAL = 402,                 /* tRETVAL  */
    tSAFEARRAY = 403,              /* tSAFEARRAY  */
    tSHORT = 404,                  /* tSHORT  */
    tSIGNED = 405,                 /* tSIGNED  */
    tSINGLENODE = 406,             /* tSINGLENODE  */
    tSIZEIS = 407,                 /* tSIZEIS  */
    tSIZEOF = 408,                 /* tSIZEOF  */
    tSMALL = 409,                  /* tSMALL  */
    tSOURCE = 410,                 /* tSOURCE  */
    tSTATIC = 411,                 /* tSTATIC  */
    tSTDCALL = 412,                /* tSTDCALL  */
    tSTRICTCONTEXTHANDLE = 413,    /* tSTRICTCONTEXTHANDLE  */
    tSTRING = 414,                 /* tSTRING  */
    tSTRUCT = 415,                 /* tSTRUCT  */
    tSWITCH = 416,                 /* tSWITCH  */
    tSWITCHIS = 417,               /* tSWITCHIS  */
    tSWITCHTYPE = 418,             /* tSWITCHTYPE  */
    tTHREADING = 419,              /* tTHREADING  */
    tTRANSMITAS = 420,             /* tTRANSMITAS  */
    tTRUE = 421,                   /* tTRUE  */
    tTYPEDEF = 422,                /* tTYPEDEF  */
    tUIDEFAULT = 423,              /* tUIDEFAULT  */
    tUNION = 424,                  /* tUNION  */
    tUNIQUE = 425,                 /* tUNIQUE  */
    tUNSIGNED = 426,               /* tUNSIGNED  */
    tUSESGETLASTERROR = 427,       /* tUSESGETLASTERROR  */
    tUSERMARSHAL = 428,            /* tUSERMARSHAL  */
    tUUID = 429,                   /* tUUID  */
    tV1ENUM = 430,                 /* tV1ENUM  */
    tVARARG = 431,                 /* tVARARG  */
    tVERSION = 432,                /* tVERSION  */
    tVIPROGID = 433,               /* tVIPROGID  */
    tVOID = 434,                   /* tVOID  */
    tWCHAR = 435,                  /* tWCHAR  */
    tWIREMARSHAL = 436,            /* tWIREMARSHAL  */
    tAPARTMENT = 437,              /* tAPARTMENT  */
    tNEUTRAL = 438,                /* tNEUTRAL  */
    tSINGLE = 439,                 /* tSINGLE  */
    tFREE = 440,                   /* tFREE  */
    tBOTH = 441,                   /* tBOTH  */
    CAST = 442,                    /* CAST  */
    PPTR = 443,                    /* PPTR  */
    POS = 444,                     /* POS  */
    NEG = 445,                     /* NEG  */
    ADDRESSOF = 446                /* ADDRESSOF  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 142 "/home/moebius/reactos/sdk/tools/widl/parser.y"

	attr_t *attr;
	attr_list_t *attr_list;
	str_list_t *str_list;
	expr_t *expr;
	expr_list_t *expr_list;
	type_t *type;
	var_t *var;
	var_list_t *var_list;
	declarator_t *declarator;
	declarator_list_t *declarator_list;
	statement_t *statement;
	statement_list_t *stmt_list;
	warning_t *warning;
	warning_list_t *warning_list;
	ifref_t *ifref;
	ifref_list_t *ifref_list;
	char *str;
	UUID *uuid;
	unsigned int num;
	double dbl;
	interface_info_t ifinfo;
	typelib_t *typelib;
	struct _import_t *import;
	struct _decl_spec_t *declspec;
	enum storage_class stgclass;

#line 283 "/home/moebius/reactos/output-posix-amd64/host-tools/bin/sdk/tools/widl/parser.tab.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE parser_lval;


int parser_parse (void);


#endif /* !YY_PARSER_HOME_MOEBIUS_REACTOS_OUTPUT_POSIX_AMD64_HOST_TOOLS_BIN_SDK_TOOLS_WIDL_PARSER_TAB_H_INCLUDED  */
