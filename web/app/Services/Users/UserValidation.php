<?php

declare(strict_types=1);

namespace Wikijump\Services\Users;

use Wikijump\Models\User;
use Illuminate\Contracts\Validation\Rule as RuleContract;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Validator;
use Illuminate\Validation\Rule;
use Illuminate\Validation\Rules\Password;
use Wikidot\Utils\WDStringUtils;

/**
 * Static class holding methods useful for validating user data,
 * such as the usernames and passwords for new users.
 */
final class UserValidation
{
    private function __construct()
    {
    }

    /**
     * Checks if a username is valid. The username must be unique.
     *
     * @param string $name The username to check.
     * @param User|null $ignore_user A user to ignore for uniqueness checks, if any.
     */
    public static function isValidUsername(string $name, ?User $ignore_user = null): bool
    {
        $forbidden = config('wikijump.forbidden_usernames');

        foreach ($forbidden as $pattern) {
            if (preg_match($pattern, $name) !== 0) {
                return false;
            }
        }

        // check if username is valid, but that's not the only thing we need to check

        $name_valid = static::validate($name, [
            'required',
            'string',
            'max:255',
            'min:3',
            isset($ignore_user)
                ? Rule::unique('users')->ignore($ignore_user->id)
                : 'unique:users,username',
        ]);

        if (!$name_valid) {
            return false;
        }

        // now we have to check if the slug version of the username is valid

        $slug = WDStringUtils::toUnixName($name);

        return static::validate($slug, [
            'required',
            'string',
            'max:255',
            'min:3',
            isset($ignore_user)
                ? Rule::unique('users')->ignore($ignore_user->id)
                : 'unique:users,slug',
        ]);
    }

    /**
     * Checks if an email is valid. The email must be unique.
     *
     * @param string $email The email to check.
     * @param User|null $ignore_user A user to ignore for uniqueness checks, if any.
     */
    public static function isValidEmail(string $email, ?User $ignore_user = null): bool
    {
        return static::validate($email, [
            'required',
            'string',
            'email',
            'max:255',
            isset($ignore_user)
                ? Rule::unique('users')->ignore($ignore_user->id)
                : 'unique:users,email',
        ]);
    }

    /**
     * Checks if an email has already been taken.
     *
     * @param string $email The email to check.
     */
    public static function isEmailTaken(string $email): bool
    {
        return !static::validate($email, [
            'required',
            'string',
            'email',
            'max:255',
            'unique:users,email',
        ]);
    }

    /**
     * Checks if a password is valid.
     *
     * @param string $password The password to check.
     */
    public static function isValidPassword(string $password): bool
    {
        return static::validate($password, Password::min(8)->uncompromised(10));
    }

    /**
     * Checks if a real name is valid.
     *
     * @param string $real_name The name to check.
     */
    public static function isValidRealName(string $real_name): bool
    {
        return static::validate($real_name, ['required', 'string', 'max:255']);
    }

    /**
     * Checks if an avatar image is valid.
     *
     * @param UploadedFile $image The image to check.
     */
    public static function isValidAvatar(UploadedFile $image): bool
    {
        return static::validate($image, [
            'required',
            'image',
            'mimes:jpg,jpeg,png,webp',
            'max:1024',
        ]);
    }

    /**
     * @param mixed $input
     * @param array|string|RuleContract $rule
     */
    private static function validate($input, $rule): bool
    {
        $validator = Validator::make(['value' => $input], ['value' => $rule]);
        return !$validator->fails();
    }
}
