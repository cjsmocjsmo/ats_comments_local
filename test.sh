#!/bin/bash



curl http://192.168.0.91:8080/addcom/Herman%20Munster/herman@monster.com/this%20is%20the%20first%20comment%20lets%20see%20if%20it%20works/5;

curl http://192.168.0.91:8080/addcom/Casper%20The%20Ghost/casper@monster.com/this%20is%20another%20commit%20from%20casper/4;

curl http://192.168.0.91:8080/allcom;

curl http://192.168.0.91:8080/addesti/foo%20bar/814%20Hull%20Ave/Port%20Orchard/903-465-7811/foo@gmail.com/A%20tree%20fell%20I%20need%20help/01-02-2023;

curl http://192.168.0.91:8080/addesti/charlie%20smotherman/924%20hull%20ave/port%20orchard/903-465-7811/charlie@gmail.com/great%20job"

curl http://192.168.0.91:8080/allesti;

# curl http://192.168.0.91:8080/addcom/Herman Munster/herman@monster.com/this is the first comment lets see if it works;


# SENDMAIL="/usr/share/sendmail/sendmail/sendmail"
# $SENDMAIL \
#     -etype "esti" \
#     -name "Herman Munster" \
#     -address "555 Mockingbird Lane" \
#     -city "Mockingbird Heights" \
#     -phone "555-555-5555" \
#     -email "herman@monster.com" \
#     -comment "This is the first comment lets see if it works" \
#     -intake "01-02-2023";
    