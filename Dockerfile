FROM weetmuts/wmbusmeters:release-1.6.0

RUN apk add --no-cache python3
ADD files/ /wmbusmeters/
