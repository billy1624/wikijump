<?php


namespace DB;

use BaseDBPeer;

/**
 * Base peer class mapped to the database table category.
 */
class CategoryPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='category';
        $this->objectName='DB\\Category';
        $this->primaryKeyName = 'category_id';
        $this->fieldNames = array( 'category_id' ,  'site_id' ,  'name' ,  'theme_default' ,  'theme_id' ,  'theme_external_url' ,  'permissions_default' ,  'permissions' ,  'license_default' ,  'license_id' ,  'license_other' ,  'nav_default' ,  'top_bar_page_name' ,  'side_bar_page_name' ,  'template_id' ,  'per_page_discussion' ,  'per_page_discussion_default' ,  'rating' ,  'category_template_id' ,  'autonumerate' ,  'page_title_template' ,  'enable_pingback_out' ,  'enable_pingback_in' );
        $this->fieldTypes = array( 'category_id' => 'serial',  'site_id' => 'int',  'name' => 'varchar(80)',  'theme_default' => 'boolean',  'theme_id' => 'int',  'theme_external_url' => 'varchar(512)',  'permissions_default' => 'boolean',  'permissions' => 'varchar(200)',  'license_default' => 'boolean',  'license_id' => 'int',  'license_other' => 'varchar(300)',  'nav_default' => 'boolean',  'top_bar_page_name' => 'varchar(128)',  'side_bar_page_name' => 'varchar(128)',  'template_id' => 'int',  'per_page_discussion' => 'boolean',  'per_page_discussion_default' => 'boolean',  'rating' => 'varchar(10)',  'category_template_id' => 'int',  'autonumerate' => 'boolean',  'page_title_template' => 'varchar(256)',  'enable_pingback_out' => 'boolean',  'enable_pingback_in' => 'boolean');
        $this->defaultValues = array( 'theme_default' => 'true',  'permissions_default' => 'true',  'license_default' => 'true',  'nav_default' => 'true',  'per_page_discussion_default' => 'true',  'autonumerate' => 'false',  'enable_pingback_out' => 'false',  'enable_pingback_in' => 'false');
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            self::$peerInstance = new CategoryPeer();
        }
        return self::$peerInstance;
    }
}
