#!/bin/bash


# A1=http://192.168.0.91:8080/addcom
# A2='Herman Munster'
# A3='herman@monster.com'
# A4='This is the first comment lets see if it works'
# A5='5'
# AURL="${A1}/${A2}/${A3}/${A4}/${A5}"
# encoded_url=$(encode_uri_component "$AURL")
# curl $encoded_url

# curl this%20is%20the%20first%20comment%20lets%20see%20if%20it%20works/5;

curl http://atstest.xyz/addcom/Casper%20The%20Ghost/casper@monster.com/this%20is%20another%20commit%20from%20casper/4;

# curl http://192.168.0.91:8080/allcom;

# curl http://192.168.0.91:8080/addesti/foo%20bar/814%20Hull%20Ave/Port%20Orchard/903-465-7811/foo@gmail.com/A%20tree%20fell%20I%20need%20help/01-02-2023;

# curl http://192.168.0.91:8080/addesti/charlie%20smotherman/924%20hull%20ave/port%20orchard/903-465-7811/charlie@gmail.com/great%20job"

# curl http://192.168.0.91:8080/allesti;

# # curl http://192.168.0.91:8080/addcom/Herman Munster/herman@monster.com/this is the first comment lets see if it works;

# curl --request POST "https://atstest.xyz/addcom" \
#     --verbose \
#     --header "Content-Type: multipart/form-data" \
#     -F name='Herman Munster' \
#     -F email='herman@gmail.com' \
#     -F comment='This is the first comment lets see if it works' \
#     -F rating='5' \
    # -F "filepicker=@test.webp"

# curl --request POST "https://atstest.xyz/addesti" \
#     --verbose \
#     --header "Content-Type: multipart/form-data" \
#     -F name='Herman Munster' \
#     -F address='555 Mockingbird Lane' \
#     -F city="Mockingbird Heights" \
#     -F phone='555-555-5555' \
#     -F email='herman@monster.com' \
#     -F comment='This is the first comment lets see if it works' \
#     -F reqdate='01-02-2023' \
    # --form "filepicker=@/home/charliepi/Downloads/test.webp"
    
#    info!("name: {:#?}", name);
#     info!("address: {:#?}", address);
#     info!("city: {:#?}", city);
#     info!("phone: {:#?}", phone);
#     info!("email: {:#?}", email);
#     info!("comment: {:#?}", comment);
#     info!("reqdate: {:#?}", reqdate);
#     info!("media_path: {:#?}", media_path);
 