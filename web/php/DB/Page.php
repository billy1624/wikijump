<?php

namespace Wikidot\DB;

use Ozone\Framework\Database\Criteria;
use Wikijump\Models\User;
use Wikijump\Services\Deepwell\PageService;

/**
 * Object Model Class.
 *
 */
class Page extends PageBase
{
    protected static $_titleTemplate = [];

    public function getMetadata()
    {
        return $this->getCurrentRevision()->getMetadata();
    }

    // TODO: remove
    public function getSource(): string
    {
        $contents = PageService::getLatestContents($this->getPageId(), ['wikitext']);
        return $contents['wikitext'];
    }

    // TODO: remove
    public function getCompiled(): string
    {
        $contents = PageService::getLatestContents($this->getPageId(), ['compiled_html']);
        return $contents['compiled_html'];
    }

    public function getCurrentRevision()
    {
        $c = new Criteria();
        $c->add("revision_id", $this->getRevisionId());
        return PageRevisionPeer::instance()->selectOne($c);
    }

    public function getFiles()
    {
        $q = "SELECT * FROM file WHERE page_id='" . $this->getPageId() . "' ORDER BY filename, file_id DESC";
        $c = new Criteria();
        $c->setExplicitQuery($q);

        return FilePeer::instance()->select($c);
    }

    public function getCategoryName()
    {
        $unixName = $this->getUnixName();
        if (strpos($unixName, ":") != false) {
            $tmp0 = explode(':', $unixName);
            $categoryName = $tmp0[0];
        } else {
            $categoryName = "_default";
        }
        return $categoryName;
    }

    public function getCategory()
    {
        $categoryId = $this->getCategoryId();
        $siteId = $this->getSiteId();

        return CategoryPeer::instance()->selectById($categoryId, $siteId);
    }

    public function getTitleOrUnixName()
    {
        $title = $this->getTitle();
        if ($title === null || $title === '') {
            $title = ucfirst(str_replace("-", " ", preg_replace("/^[a-z0-9\-]+:/i", '', $this->getUnixName())));
        }
        return $title;
    }

    public function getLastEditUserOrString()
    {
        $user = $this->getLastEditUser();
        if ($user == null) {
            return $this->getLastEditUserString();
        } else {
            return $user;
        }
    }

    public function getLastEditUser()
    {
        if ($this->getLastEditUserId() == User::ANONYMOUS_USER) {
            return null;
        }
        return User::find($this->getLastEditUserId());
    }

    public function getSite()
    {
        return SitePeer::instance()->selectByPrimaryKey($this->getSiteId());
    }

    public function getTags()
    {
        return PagePeer::getTags($this->getPageId());
    }

    public function getTitle()
    {
        $categoryId = $this->getCategoryId();
        if ($categoryId) {
            if (!array_key_exists($categoryId, self::$_titleTemplate)) {
                /* Check for template. */
                $c = new Criteria();
                $templateUnixName = '_titletemplate';
                if ($this->getCategoryName() != '_default') {
                    $templateUnixName = $this->getCategoryName() . ':' . $templateUnixName;
                }

                $c->add('unix_name', $templateUnixName);
                $c->add('site_id', $this->getSiteId());
                $templatePage = PagePeer::instance()->selectOne($c);
                if ($templatePage) {
                    $templateSource = $templatePage->getSource();
                    if (strlen($templateSource) > 0 && strlen($templateSource) < 200 && !strpos($templateSource, "\n")) {
                        self::$_titleTemplate[$categoryId] = $templateSource;
                    } else {
                        self::$_titleTemplate[$categoryId] = false;
                    }
                } else {
                    self::$_titleTemplate[$categoryId] = false;
                }
            }
            $titleTemplate = self::$_titleTemplate[$categoryId];
            if ($titleTemplate) {
                /* Process the template. */
                $b = $titleTemplate;
                $b = str_replace('%%page_unix_name%%', preg_replace('/^[a-z0-9]+:/', '', $this->getUnixName()), $b);
                $b = str_replace('%%title%%', parent::getTitle(), $b);
                return $b;
            }
        }
        return parent::getTitle();
    }

    public function getTitleRaw()
    {
        return parent::getTitle();
    }
}
