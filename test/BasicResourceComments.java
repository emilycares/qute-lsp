//linecomment
/*c*/package/*c*/ ch/*c*/./*c*/micmine/*c*/;/*c*/
//linecomment

/*c<*/import/*c<*/ /*c<*/jakarta/*c<*/./*c<*/inject/*c<*/.Inject/*c<*/;
//linecomment
/*c*/import/*c*/ /*c*/jakarta/*c*/./*c*/ws/*c*/./*c*/rs/*c*/./*c*/GET/*c*/;/*c*/
//linecomment
/*c*/import/*c*/ /*c*/jakarta/*c*/./*c*/ws/*c*/./*c*/rs/*c*/./*c*/Path/*c*/;/*c*/
//linecomment
/*c*/import/*c*/ /*c*/jakarta/*c*/./*c*/ws/*c*/./*c*/rs/*c*/./*c*/Produces/*c*/;/*c*/
//linecomment
/*c*/import/*c*/ /*c*/jakarta/*c*/./*c*/ws/*c*/./*c*/rs/*c*/./*c*/core/*c*/./*c*/MediaType/*c*/;/*c*/
//linecomment

/*c*/import/*c*/ io.quarkus.qute.TemplateInstance/*c*/;/*c*/
//linecomment
/*c*/import/*c*/ io.quarkus.qute.Template/*c*/;/*c*/
//linecomment

//linecomment
/*c*/@/*c*/Slf4j/*c*/
//linecomment
/*c*/@/*c*/Path/*c*/(/*c*/"/hello"/*c*/)/*c*/
//linecomment
/*c*/@/*c*/Produces/*c*/(/*c*/MediaType/*c*/./*c*/TEXT_PLAIN/*c*/)/*c*/
//linecomment
/*c*/@/*c*/ApplicationScoped/*c*/
//linecomment
/*c*/@/*c*/WithSession/*c*/
//linecomment
/*c*/public/*c*/ class/*c*/ BasicResource/*c*/ extends/*c*/ asd/*c*/ implements/*c*/ sdf/*c*/ {/*c*/
//linecomment

//linecomment
    /*c*/@/*c*/Inject/*c*/
//linecomment
    /*c*/Template/*c*/ hello/*c*/;/*c*/
//linecomment

//linecomment
    /*c*/@/*c*/GET/*c*/
//linecomment
    /*c*/@/*c*/Produces/*c*/(/*c*/MediaType/*c*/./*c*/TEXT_HTML/*c*/)/*c*/
//linecomment
    /*c*/public/*c*/ TemplateInstance/*c*/ hello/*c*/()/*c*/ {/*c*/
//linecomment
            /*c*/return/*c*/ hello/*c*/./*c*/data/*c*/(/*c*/"name"/*c*/,/*c*/ "micmine"/*c*/)/*c*/;/*c*/
//linecomment
    /*c*/}
//linecomment

//linecomment
    /*c*/@/*c*/GET/*c*/
//linecomment
    /*c*/@/*c*/Produces/*c*/(/*c*/MediaType/*c*/./*c*/TEXT_HTML/*c*/)/*c*/
//linecomment
    /*c*/@/*c*/Path/*c*/(/*c*/"/customer/{name}"/*c*/)/*c*/
//linecomment
    /*c*/public/*c*/ TemplateInstance/*c*/ customer/*c*/(@/*c*/PathParam/*c*/(/*c*/"name"/*c*/)/*c*/ String/*c*/ name/*c*/)/*c*/ {/*c*/
//linecomment
            /*c*/return/*c*/ hello/*c*/./*c*/data/*c*/(/*c*/"name"/*c*/,/*c*/ name/*c*/)/*c*/;/*c*/
//linecomment
    /*c*/}/*c*/
//linecomment

//linecomment
    /*c*/@/*c*/PUT/*c*/
//linecomment
    /*c*/@/*c*/Produces/*c*/(/*c*/MediaType/*c*/./*c*/APPLICATION_JSON/*c*/)/*c*/
//linecomment
    /*c*/@/*c*/Path/*c*/(/*c*/"/customer/{name}/{sufix}"/*c*/)/*c*/
//linecomment
    /*c*/public/*c*/ TemplateInstance/*c*/ customer_other/*c*/(@/*c*/PathParam/*c*/(/*c*/"name"/*c*/)/*c*/ String/*c*/ name/*c*/,/*c*/ @/*c*/PathParam/*c*/(/*c*/"sufix"/*c*/)/*c*/ int/*c*/ name/*c*/)/*c*/ {/*c*/
//linecomment
    /*c*/}/*c*/
//linecomment

//linecomment
    /*c*/@/*c*/PUT/*c*/
//linecomment
    /*c*/@/*c*/Produces/*c*/(/*c*/MediaType/*c*/./*c*/APPLICATION_JSON/*c*/)/*c*/
//linecomment
    /*c*/@/*c*/Path/*c*/(/*c*/"/customer/{name}/{sufix}"/*c*/)/*c*/
//linecomment
    /*c*/public/*c*/ TemplateInstance/*c*/ no_annotation_path_param/*c*/(/*c*/String/*c*/ name/*c*/,/*c*/ int/*c*/ sufix/*c*/)/*c*/ {/*c*/
//linecomment
    /*c*/}/*c*/
//linecomment
}/*c*/
//linecomment
