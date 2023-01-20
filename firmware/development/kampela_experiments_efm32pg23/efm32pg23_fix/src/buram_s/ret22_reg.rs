#[doc = "Register `RET22_REG` reader"]
pub struct R(crate::R<RET22_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RET22_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RET22_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RET22_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RET22_REG` writer"]
pub struct W(crate::W<RET22_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RET22_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RET22_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RET22_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETREG` reader - Latch based Retention register"]
pub type RETREG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RETREG` writer - Latch based Retention register"]
pub type RETREG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RET22_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Latch based Retention register"]
    #[inline(always)]
    pub fn retreg(&self) -> RETREG_R {
        RETREG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Latch based Retention register"]
    #[inline(always)]
    #[must_use]
    pub fn retreg(&mut self) -> RETREG_W<0> {
        RETREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ret22_reg](index.html) module"]
pub struct RET22_REG_SPEC;
impl crate::RegisterSpec for RET22_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ret22_reg::R](R) reader structure"]
impl crate::Readable for RET22_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ret22_reg::W](W) writer structure"]
impl crate::Writable for RET22_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RET22_REG to value 0"]
impl crate::Resettable for RET22_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
