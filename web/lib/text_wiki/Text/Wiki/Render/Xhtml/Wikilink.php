<?php
// vim: set expandtab tabstop=4 shiftwidth=4 softtabstop=4:
/**
 * Wikilink rule end renderer for Xhtml
 *
 * PHP versions 4 and 5
 *
 * @category   Text
 * @package    Text_Wiki
 * @author     Paul M. Jones <pmjones@php.net>
 * @author Michal Frackowiak
 * @license    http://www.gnu.org/copyleft/lesser.html  LGPL License 2.1
 * @version    $Id$
 * @link       http://pear.php.net/package/Text_Wiki
 */

/**
 * This class renders wiki links in XHTML.
 *
 * @category   Text
 * @package    Text_Wiki
 * @author     Paul M. Jones <pmjones@php.net>
 * @author Michal Frackowiak
 * @license    http://www.gnu.org/copyleft/lesser.html  LGPL License 2.1
 * @version    Release: @package_version@
 * @link       http://pear.php.net/package/Text_Wiki
 */

use Wikidot\DB\PagePeer;
use Wikidot\Utils\GlobalProperties;

class Text_Wiki_Render_Xhtml_Wikilink extends Text_Wiki_Render {

    public $conf = array(
        'pages' => array(), // set to null or false to turn off page checks
        'view_url' => 'http://example.com/laravel.php?page=%s',
        'new_url'  => 'http://example.com/new.php?page=%s',
        'new_text' => '?',
        'new_text_pos' => 'after', // 'before', 'after', or null/false
        'css' => null,
        'css_new' => null,
        'exists_callback' => null // call_user_func() callback
    );

    /**
    *
    * Renders a token into XHTML.
    *
    * @access public
    *
    * @param array $options The "options" portion of the token (second
    * element).
    *
    * @return string The text rendered from the token options.
    *
    */

    function token($options)
    {
        /**
         * NOTE: This renderer handles both the Wikilink and Freelink parse objects.
         * @see Text_Wiki_Render_Xhtml_Freelink
         *
         * @var $site
         * @var $text
         * @var $page
         * @var $textFromTitle
         * @var $anchor
         * @var $nonbr
         */
        extract($options);
        if($site){

            $o = '<a href="'. GlobalProperties::$HTTP_SCHEMA .'://'.$site.'.'.GlobalProperties::$URL_DOMAIN.'/'.$page.'">';
            $o .= $text;
            $o .= '</a>';
            return $o;
        }
        // make nice variable names (page, anchor, text)

        // is there a "page existence" callback?
        // we need to access it directly instead of through
        // getConf() because we'll need a reference (for
        // object instance method callbacks).
        if (isset($this->conf['exists_callback'])) {
            $callback =& $this->conf['exists_callback'];
        } else {
        	$callback = false;
        }

        if ($callback) {
            // use the callback function
            $exists = call_user_func($callback, $page);
        } else {
            // no callback, go to the naive page array.
            $list =& $this->getConf('pages');
            if (is_array($list)) {
                // yes, check against the page list
                $exists = in_array($page, $list);
            } else {
                // no, assume it exists
                $exists = true;
            }
        }

		if($exists && $textFromTitle){
			// get displayed text from the page title
			$pageObj = PagePeer::instance()->selectByPrimaryKey($exists);
			$text = $pageObj->getTitleOrUnixName();
		}

		if(!$exists && $textFromTitle){
			$text = $page;
		}

        // convert *after* checking against page names so as not to mess
        // up what the user typed and what we're checking.
        $page = htmlspecialchars(trim($page));
        $anchor = htmlspecialchars(trim($anchor));
        $text = htmlspecialchars(trim($text));

       	if($nonbr){
       		$text = str_replace(' ', '&nbsp;', $text);
       	}

        if($this->wiki->vars['internalLinks'] === null){
            $this->wiki->vars['internalLinks'] = [];
        }
        $this->wiki->vars['internalLinks'][$page] = $page;

        // does the page exist?
        if ($exists) {

            // PAGE EXISTS.

            // link to the page view, but we have to build
            // the HREF.  we support both the old form where
            // the page always comes at the end, and the new
            // form that uses %s for sprintf()
            $href = $this->getConf('view_url');

            if (strpos($href, '%s') === false) {
                // use the old form (page-at-end)
                $href = $href . $page . $anchor;
            } else {
                // use the new form (sprintf format string)
                $href = sprintf($href, $page . $anchor);
            }

            // get the CSS class and generate output
            $css = $this->formatConf(' class="%s"', 'css');
            $output = "<a$css href=\"$href\">$text</a>";

        } else {

            // link to a create-page url, but only if new_url is set
            $href = $this->getConf('new_url', null);

            // set the proper HREF
            if (! $href || trim($href) == '') {

                // no useful href, return the text as it is
                $output = $text;

            } else {

                // yes, link to the new-page href, but we have to build
                // it.  we support both the old form where
                // the page always comes at the end, and the new
                // form that uses sprintf()
                if (strpos($href, '%s') === false) {
                    // use the old form
                    $href = $href . $page;
                } else {
                    // use the new form
                    $href = sprintf($href, $page);
                }
            }

            // get the appropriate CSS class and new-link text
            $css = $this->formatConf(' class="%s"', 'css_new');
            $new = $this->getConf('new_text');

            // what kind of linking are we doing?
            $pos = $this->getConf('new_text_pos');
            if (! $pos || ! $new) {
                // no position (or no new_text), use css only on the page name
                $output = "<a$css href=\"$href\">$text</a>";
            } elseif ($pos == 'before') {
                // use the new_text BEFORE the page name
                $output = "<a$css href=\"$href\">$new</a>$text";
            } else {
                // default, use the new_text link AFTER the page name
                $output = "$text<a$css href=\"$href\">$new</a>";
            }
        }
        return $output;
    }
}
