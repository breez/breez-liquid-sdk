// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'model.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$LnUrlPayResult {
  Object get data => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $LnUrlPayResultCopyWith<$Res> {
  factory $LnUrlPayResultCopyWith(LnUrlPayResult value, $Res Function(LnUrlPayResult) then) =
      _$LnUrlPayResultCopyWithImpl<$Res, LnUrlPayResult>;
}

/// @nodoc
class _$LnUrlPayResultCopyWithImpl<$Res, $Val extends LnUrlPayResult>
    implements $LnUrlPayResultCopyWith<$Res> {
  _$LnUrlPayResultCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointSuccessImplCopyWith(_$LnUrlPayResult_EndpointSuccessImpl value,
          $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) then) =
      __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPaySuccessData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointSuccessImpl>
    implements _$$LnUrlPayResult_EndpointSuccessImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl(
      _$LnUrlPayResult_EndpointSuccessImpl _value, $Res Function(_$LnUrlPayResult_EndpointSuccessImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointSuccessImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPaySuccessData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointSuccessImpl extends LnUrlPayResult_EndpointSuccess {
  const _$LnUrlPayResult_EndpointSuccessImpl({required this.data}) : super._();

  @override
  final LnUrlPaySuccessData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointSuccess(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointSuccessImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointSuccessImplCopyWith<_$LnUrlPayResult_EndpointSuccessImpl> get copyWith =>
      __$$LnUrlPayResult_EndpointSuccessImplCopyWithImpl<_$LnUrlPayResult_EndpointSuccessImpl>(
          this, _$identity);
}

abstract class LnUrlPayResult_EndpointSuccess extends LnUrlPayResult {
  const factory LnUrlPayResult_EndpointSuccess({required final LnUrlPaySuccessData data}) =
      _$LnUrlPayResult_EndpointSuccessImpl;
  const LnUrlPayResult_EndpointSuccess._() : super._();

  @override
  LnUrlPaySuccessData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointSuccessImplCopyWith<_$LnUrlPayResult_EndpointSuccessImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_EndpointErrorImplCopyWith(
          _$LnUrlPayResult_EndpointErrorImpl value, $Res Function(_$LnUrlPayResult_EndpointErrorImpl) then) =
      __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_EndpointErrorImpl>
    implements _$$LnUrlPayResult_EndpointErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl(
      _$LnUrlPayResult_EndpointErrorImpl _value, $Res Function(_$LnUrlPayResult_EndpointErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_EndpointErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_EndpointErrorImpl extends LnUrlPayResult_EndpointError {
  const _$LnUrlPayResult_EndpointErrorImpl({required this.data}) : super._();

  @override
  final LnUrlErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.endpointError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_EndpointErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_EndpointErrorImplCopyWith<_$LnUrlPayResult_EndpointErrorImpl> get copyWith =>
      __$$LnUrlPayResult_EndpointErrorImplCopyWithImpl<_$LnUrlPayResult_EndpointErrorImpl>(this, _$identity);
}

abstract class LnUrlPayResult_EndpointError extends LnUrlPayResult {
  const factory LnUrlPayResult_EndpointError({required final LnUrlErrorData data}) =
      _$LnUrlPayResult_EndpointErrorImpl;
  const LnUrlPayResult_EndpointError._() : super._();

  @override
  LnUrlErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_EndpointErrorImplCopyWith<_$LnUrlPayResult_EndpointErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  factory _$$LnUrlPayResult_PayErrorImplCopyWith(
          _$LnUrlPayResult_PayErrorImpl value, $Res Function(_$LnUrlPayResult_PayErrorImpl) then) =
      __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({LnUrlPayErrorData data});
}

/// @nodoc
class __$$LnUrlPayResult_PayErrorImplCopyWithImpl<$Res>
    extends _$LnUrlPayResultCopyWithImpl<$Res, _$LnUrlPayResult_PayErrorImpl>
    implements _$$LnUrlPayResult_PayErrorImplCopyWith<$Res> {
  __$$LnUrlPayResult_PayErrorImplCopyWithImpl(
      _$LnUrlPayResult_PayErrorImpl _value, $Res Function(_$LnUrlPayResult_PayErrorImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? data = null,
  }) {
    return _then(_$LnUrlPayResult_PayErrorImpl(
      data: null == data
          ? _value.data
          : data // ignore: cast_nullable_to_non_nullable
              as LnUrlPayErrorData,
    ));
  }
}

/// @nodoc

class _$LnUrlPayResult_PayErrorImpl extends LnUrlPayResult_PayError {
  const _$LnUrlPayResult_PayErrorImpl({required this.data}) : super._();

  @override
  final LnUrlPayErrorData data;

  @override
  String toString() {
    return 'LnUrlPayResult.payError(data: $data)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$LnUrlPayResult_PayErrorImpl &&
            (identical(other.data, data) || other.data == data));
  }

  @override
  int get hashCode => Object.hash(runtimeType, data);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$LnUrlPayResult_PayErrorImplCopyWith<_$LnUrlPayResult_PayErrorImpl> get copyWith =>
      __$$LnUrlPayResult_PayErrorImplCopyWithImpl<_$LnUrlPayResult_PayErrorImpl>(this, _$identity);
}

abstract class LnUrlPayResult_PayError extends LnUrlPayResult {
  const factory LnUrlPayResult_PayError({required final LnUrlPayErrorData data}) =
      _$LnUrlPayResult_PayErrorImpl;
  const LnUrlPayResult_PayError._() : super._();

  @override
  LnUrlPayErrorData get data;
  @JsonKey(ignore: true)
  _$$LnUrlPayResult_PayErrorImplCopyWith<_$LnUrlPayResult_PayErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$SdkEvent {}

/// @nodoc
abstract class $SdkEventCopyWith<$Res> {
  factory $SdkEventCopyWith(SdkEvent value, $Res Function(SdkEvent) then) =
      _$SdkEventCopyWithImpl<$Res, SdkEvent>;
}

/// @nodoc
class _$SdkEventCopyWithImpl<$Res, $Val extends SdkEvent> implements $SdkEventCopyWith<$Res> {
  _$SdkEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentFailedImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentFailedImplCopyWith(
          _$SdkEvent_PaymentFailedImpl value, $Res Function(_$SdkEvent_PaymentFailedImpl) then) =
      __$$SdkEvent_PaymentFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentFailedImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentFailedImpl>
    implements _$$SdkEvent_PaymentFailedImplCopyWith<$Res> {
  __$$SdkEvent_PaymentFailedImplCopyWithImpl(
      _$SdkEvent_PaymentFailedImpl _value, $Res Function(_$SdkEvent_PaymentFailedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentFailedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentFailedImpl extends SdkEvent_PaymentFailed {
  const _$SdkEvent_PaymentFailedImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentFailed(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentFailedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentFailedImplCopyWith<_$SdkEvent_PaymentFailedImpl> get copyWith =>
      __$$SdkEvent_PaymentFailedImplCopyWithImpl<_$SdkEvent_PaymentFailedImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentFailed extends SdkEvent {
  const factory SdkEvent_PaymentFailed({required final Payment details}) = _$SdkEvent_PaymentFailedImpl;
  const SdkEvent_PaymentFailed._() : super._();

  Payment get details;
  @JsonKey(ignore: true)
  _$$SdkEvent_PaymentFailedImplCopyWith<_$SdkEvent_PaymentFailedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentPendingImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentPendingImplCopyWith(
          _$SdkEvent_PaymentPendingImpl value, $Res Function(_$SdkEvent_PaymentPendingImpl) then) =
      __$$SdkEvent_PaymentPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentPendingImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentPendingImpl>
    implements _$$SdkEvent_PaymentPendingImplCopyWith<$Res> {
  __$$SdkEvent_PaymentPendingImplCopyWithImpl(
      _$SdkEvent_PaymentPendingImpl _value, $Res Function(_$SdkEvent_PaymentPendingImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentPendingImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentPendingImpl extends SdkEvent_PaymentPending {
  const _$SdkEvent_PaymentPendingImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentPending(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentPendingImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentPendingImplCopyWith<_$SdkEvent_PaymentPendingImpl> get copyWith =>
      __$$SdkEvent_PaymentPendingImplCopyWithImpl<_$SdkEvent_PaymentPendingImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentPending extends SdkEvent {
  const factory SdkEvent_PaymentPending({required final Payment details}) = _$SdkEvent_PaymentPendingImpl;
  const SdkEvent_PaymentPending._() : super._();

  Payment get details;
  @JsonKey(ignore: true)
  _$$SdkEvent_PaymentPendingImplCopyWith<_$SdkEvent_PaymentPendingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentRefundedImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentRefundedImplCopyWith(
          _$SdkEvent_PaymentRefundedImpl value, $Res Function(_$SdkEvent_PaymentRefundedImpl) then) =
      __$$SdkEvent_PaymentRefundedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundedImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundedImpl>
    implements _$$SdkEvent_PaymentRefundedImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundedImplCopyWithImpl(
      _$SdkEvent_PaymentRefundedImpl _value, $Res Function(_$SdkEvent_PaymentRefundedImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentRefundedImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentRefundedImpl extends SdkEvent_PaymentRefunded {
  const _$SdkEvent_PaymentRefundedImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentRefunded(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentRefundedImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentRefundedImplCopyWith<_$SdkEvent_PaymentRefundedImpl> get copyWith =>
      __$$SdkEvent_PaymentRefundedImplCopyWithImpl<_$SdkEvent_PaymentRefundedImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentRefunded extends SdkEvent {
  const factory SdkEvent_PaymentRefunded({required final Payment details}) = _$SdkEvent_PaymentRefundedImpl;
  const SdkEvent_PaymentRefunded._() : super._();

  Payment get details;
  @JsonKey(ignore: true)
  _$$SdkEvent_PaymentRefundedImplCopyWith<_$SdkEvent_PaymentRefundedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentRefundPendingImplCopyWith(_$SdkEvent_PaymentRefundPendingImpl value,
          $Res Function(_$SdkEvent_PaymentRefundPendingImpl) then) =
      __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentRefundPendingImpl>
    implements _$$SdkEvent_PaymentRefundPendingImplCopyWith<$Res> {
  __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl(
      _$SdkEvent_PaymentRefundPendingImpl _value, $Res Function(_$SdkEvent_PaymentRefundPendingImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentRefundPendingImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentRefundPendingImpl extends SdkEvent_PaymentRefundPending {
  const _$SdkEvent_PaymentRefundPendingImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentRefundPending(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentRefundPendingImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentRefundPendingImplCopyWith<_$SdkEvent_PaymentRefundPendingImpl> get copyWith =>
      __$$SdkEvent_PaymentRefundPendingImplCopyWithImpl<_$SdkEvent_PaymentRefundPendingImpl>(
          this, _$identity);
}

abstract class SdkEvent_PaymentRefundPending extends SdkEvent {
  const factory SdkEvent_PaymentRefundPending({required final Payment details}) =
      _$SdkEvent_PaymentRefundPendingImpl;
  const SdkEvent_PaymentRefundPending._() : super._();

  Payment get details;
  @JsonKey(ignore: true)
  _$$SdkEvent_PaymentRefundPendingImplCopyWith<_$SdkEvent_PaymentRefundPendingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentSucceededImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentSucceededImplCopyWith(
          _$SdkEvent_PaymentSucceededImpl value, $Res Function(_$SdkEvent_PaymentSucceededImpl) then) =
      __$$SdkEvent_PaymentSucceededImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentSucceededImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentSucceededImpl>
    implements _$$SdkEvent_PaymentSucceededImplCopyWith<$Res> {
  __$$SdkEvent_PaymentSucceededImplCopyWithImpl(
      _$SdkEvent_PaymentSucceededImpl _value, $Res Function(_$SdkEvent_PaymentSucceededImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentSucceededImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentSucceededImpl extends SdkEvent_PaymentSucceeded {
  const _$SdkEvent_PaymentSucceededImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentSucceeded(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentSucceededImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentSucceededImplCopyWith<_$SdkEvent_PaymentSucceededImpl> get copyWith =>
      __$$SdkEvent_PaymentSucceededImplCopyWithImpl<_$SdkEvent_PaymentSucceededImpl>(this, _$identity);
}

abstract class SdkEvent_PaymentSucceeded extends SdkEvent {
  const factory SdkEvent_PaymentSucceeded({required final Payment details}) = _$SdkEvent_PaymentSucceededImpl;
  const SdkEvent_PaymentSucceeded._() : super._();

  Payment get details;
  @JsonKey(ignore: true)
  _$$SdkEvent_PaymentSucceededImplCopyWith<_$SdkEvent_PaymentSucceededImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  factory _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith(_$SdkEvent_PaymentWaitingConfirmationImpl value,
          $Res Function(_$SdkEvent_PaymentWaitingConfirmationImpl) then) =
      __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Payment details});
}

/// @nodoc
class __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<$Res>
    extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_PaymentWaitingConfirmationImpl>
    implements _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<$Res> {
  __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl(_$SdkEvent_PaymentWaitingConfirmationImpl _value,
      $Res Function(_$SdkEvent_PaymentWaitingConfirmationImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? details = null,
  }) {
    return _then(_$SdkEvent_PaymentWaitingConfirmationImpl(
      details: null == details
          ? _value.details
          : details // ignore: cast_nullable_to_non_nullable
              as Payment,
    ));
  }
}

/// @nodoc

class _$SdkEvent_PaymentWaitingConfirmationImpl extends SdkEvent_PaymentWaitingConfirmation {
  const _$SdkEvent_PaymentWaitingConfirmationImpl({required this.details}) : super._();

  @override
  final Payment details;

  @override
  String toString() {
    return 'SdkEvent.paymentWaitingConfirmation(details: $details)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$SdkEvent_PaymentWaitingConfirmationImpl &&
            (identical(other.details, details) || other.details == details));
  }

  @override
  int get hashCode => Object.hash(runtimeType, details);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<_$SdkEvent_PaymentWaitingConfirmationImpl>
      get copyWith =>
          __$$SdkEvent_PaymentWaitingConfirmationImplCopyWithImpl<_$SdkEvent_PaymentWaitingConfirmationImpl>(
              this, _$identity);
}

abstract class SdkEvent_PaymentWaitingConfirmation extends SdkEvent {
  const factory SdkEvent_PaymentWaitingConfirmation({required final Payment details}) =
      _$SdkEvent_PaymentWaitingConfirmationImpl;
  const SdkEvent_PaymentWaitingConfirmation._() : super._();

  Payment get details;
  @JsonKey(ignore: true)
  _$$SdkEvent_PaymentWaitingConfirmationImplCopyWith<_$SdkEvent_PaymentWaitingConfirmationImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$SdkEvent_SyncedImplCopyWith<$Res> {
  factory _$$SdkEvent_SyncedImplCopyWith(
          _$SdkEvent_SyncedImpl value, $Res Function(_$SdkEvent_SyncedImpl) then) =
      __$$SdkEvent_SyncedImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$SdkEvent_SyncedImplCopyWithImpl<$Res> extends _$SdkEventCopyWithImpl<$Res, _$SdkEvent_SyncedImpl>
    implements _$$SdkEvent_SyncedImplCopyWith<$Res> {
  __$$SdkEvent_SyncedImplCopyWithImpl(
      _$SdkEvent_SyncedImpl _value, $Res Function(_$SdkEvent_SyncedImpl) _then)
      : super(_value, _then);
}

/// @nodoc

class _$SdkEvent_SyncedImpl extends SdkEvent_Synced {
  const _$SdkEvent_SyncedImpl() : super._();

  @override
  String toString() {
    return 'SdkEvent.synced()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType && other is _$SdkEvent_SyncedImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;
}

abstract class SdkEvent_Synced extends SdkEvent {
  const factory SdkEvent_Synced() = _$SdkEvent_SyncedImpl;
  const SdkEvent_Synced._() : super._();
}
