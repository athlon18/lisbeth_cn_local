<?php

namespace Template;


use Exception;
use Google\Protobuf\Internal\MapField;
use SQLite3;

include 'vendor/autoload.php';

function dump($interface): void
{
    print_r($interface);
}

ini_set('memory_limit', '-1');
error_reporting(0);

$db = new Db();

$sql = new SQLite3("./db.s3db");
$bin_data = file_get_contents('./db.bin');

//  {"Item", "Quest", "Job", "Stat", "LogMessage", "Leve", "Mount", "book", "Reputation", "Aetheryte"}

// Book 书籍
$ret = $sql->query("SELECT id, chnname FROM item where ItemRole = 23 order by id asc");
$i = 1;
while ($row = $ret->fetchArray(SQLITE3_ASSOC)) {
    if ($i == 16) {
        $rb_book_list["Book.16"] = "烹调秘籍幻象之卷";
        $i++;
    }
    $rb_book_list["Book." . $i] = $row["ChnName"];
    $i++;
}

// 坐骑
$ret = $sql->query("SELECT id, chnname FROM MountResult order by id asc");
while ($row = $ret->fetchArray(SQLITE3_ASSOC)) {
    $rb_mount_list["Mount." . $row["Id"]] = $row["ChnName"];
}

// item 道具
$ret = $sql->query("SELECT id, chnname FROM item where chnname is not null and chnname != ''");

while ($row = $ret->fetchArray(SQLITE3_ASSOC)) {
    $rb_item_list["Item." . $row["Id"]] = $row["ChnName"];
}

$ret = $sql->query("SELECT id, ChnName FROM AetheryteResult where ChnName is not null and ChnName != ''");

while ($row = $ret->fetchArray(SQLITE3_ASSOC)) {
    $rb_aetheryte_list["Aetheryte." . $row["Id"]] = $row["ChnName"];
}

try {
    $db->mergeFromString($bin_data);
} catch (Exception $e) {
}
$olden = [];
foreach ($db->getTranslationMaps() as $v) {
    /** @var TranslationMaps $v */
    if ($v->getLanguage() == Language::Chinese) $cn = $v->getKeyToName();
    if ($v->getLanguage() == Language::English) $olden = clone $v->getKeyToName();
}


$en = new MapField($olden->getKeyType(), $olden->getValueType());
foreach ($olden as $k => $v) {
    $en->offsetSet((string)$k, (string)$v);
    if ($k == "Mount.62") { // 添加国服特有坐骑 夕
        $en->offsetSet("Mount.65", "夕");
    }
}

// 转换
foreach ($en as $k => $v) {

    if ($row = $rb_item_list[$k] ?? false) { // 道具
        $en[$k] = $row;
        continue;
    }
    if ($row = $rb_aetheryte_list[$k] ?? false) { // 水晶
        $en[$k] = $row;
        continue;
    }
    if ($row = $rb_book_list[$k] ?? false) { // 书籍
        $en[$k] = $row;
        continue;
    }

    if ($row = $rb_mount_list[$k] ?? false) { // 坐骑
        $en[$k] = $row;
        continue;
    }

    if ($row = $cn[$k] ?? false) {
        $en[$k] = $row;
        continue;
    }

//        // 未汉化数据
//        list($type, $value) = explode('.', $k);
//
//        switch ($type) {
//            //case "Book": // 书籍
//        }
//        dump($type, $value);
}

dump(count($en));
foreach ($db->getTranslationMaps() as $v) {
    /** @var TranslationMaps $v */
    if ($v->getLanguage() == Language::Chinese) {
        $v->setKeyToName($en);
        break;
    }
}
file_put_contents("./db1.bin", $db->serializeToString());
