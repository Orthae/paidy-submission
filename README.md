## Assumptions
* Security is out of scope. It is assumed that the application is running in a secure environment behind API gateway or proxy live Envoy.
* Table management is out of scope. Application support arbitrary number of tables, and they don't have to be created beforehand.
In microservices architecture, table management can be handled by a separate service.
* Menu management is out of scope. Item name is added by the staff during creation request.
