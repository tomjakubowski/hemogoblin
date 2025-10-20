#!/bin/bash

# Shouldn't crash with malformed tcp pool names but should fail

# Handle internal valgrind
IV=""
if [ "$#" == "2" ]; then
    if [ "$1" == "--internal-valgrind" ]; then
        IV=$2
    fi
fi

PATH=..:$PATH

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo partridge in a pear tree && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://hostname" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo turtle doves && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://hostname:" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo french hens && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://hostname:/" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo colly birds && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://:1/poolname" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo gold rings && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://:/poolname" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo geese a laying && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp:///poolname" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo swans a swimming && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "://example.com/poolname" 2>>${TEST_LOG}
[ "$?" != "18" ] && echo maids a milking && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "kyoto://example.com/poolname" 2>>${TEST_LOG}
[ "$?" != "18" ] && echo ladies dancing && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "://" 2>>${TEST_LOG}
[ "$?" != "18" ] && echo lords a leaping && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://example.com:blarf/poolname" 2>>${TEST_LOG}
[ "$?" != "16" ] && echo pipers piping && exit 1

$IV \
p-create ${POOL_XTRA} -t "${POOL_TYPE}" -s "${POOL_SIZE}" "tcp://10.10.10.321/poolname" 2>>${TEST_LOG}
[ "$?" != "10" ] && echo drummers drumming && exit 1

exit 0
