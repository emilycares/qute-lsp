package ch.micmine;

import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

import io.quarkus.qute.TemplateInstance;
import io.quarkus.qute.Template;

@Slf4j
@Path("/hello")
@Produces(MediaType.TEXT_PLAIN)
@ApplicationScoped
@WithSession
public class BasicResource extends asd implements sdf {

    @Inject
    Template hello;

    @GET
    @Produces(MediaType.TEXT_HTML)
    public TemplateInstance hello() {
            return hello.data("name", "micmine");
    }

    @GET
    @Produces(MediaType.TEXT_HTML)
    @Path("/customer/{name}")
    public TemplateInstance customer(@PathParam("name") String name) {
            return hello.data("name", name);
    }

    @PUT
    @Produces(MediaType.APPLICATION_JSON)
    @Path("/customer/{name}/{sufix}")
    public TemplateInstance customer_other(@PathParam("name") String name, @PathParam("sufix") int name) {
    }

    @PUT
    @Produces(MediaType.APPLICATION_JSON)
    @Path("/customer/{name}/{sufix}")
    public TemplateInstance no_anotation_path_param(String name, int sufix) {
    }
}
